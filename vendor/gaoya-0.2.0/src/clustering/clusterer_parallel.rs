use crate::clustering::QueryIndex;
use crate::minhash::{IdContainer, MinHashIndex, MinHashType};
use crossbeam_utils::atomic::AtomicCell;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::pin::Pin;
use std::ptr;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, AtomicU32, AtomicU8, Ordering};
use triomphe::Arc;

/// Implementation of a parallel,  multi-threaded clustering algorithm.

/// ClusterPointInner holds id of the point, and the pointer to the cluster.
/// Initially, `cluster` field of all points is `null_ptr()`.
/// If a point belongs to a cluster the field cluster
/// will contain a pointer to a cluster in the heap.
pub struct ClusterPointInner<Id> {
    pub id: Id,
    pub cluster: AtomicPtr<Cluster<Id>>,
}

unsafe impl<Id> Send for ClusterPointInner<Id> {}
unsafe impl<Id> Sync for ClusterPointInner<Id> {}

impl<Id> ClusterPointInner<Id>
where
    Id: Clone,
{
    pub fn new(id: Id) -> Self {
        ClusterPointInner {
            id: id,
            cluster: AtomicPtr::default(),
        }
    }

    pub fn get_cluster(&self) -> *mut Cluster<Id> {
        self.cluster.load(Ordering::Relaxed)
    }

    pub fn is_cluster_assigned(&self) -> bool {
        self.get_cluster() != null_mut()
    }

    /// Atomically assigns the cluster to this point
    /// If this point already has a cluster assignment the assignment will fail
    fn assign_cluster(
        &self,
        cluster_ptr: *mut Cluster<Id>,
    ) -> Result<*mut Cluster<Id>, *mut Cluster<Id>> {
        self.cluster.compare_exchange(
            ptr::null_mut(),
            cluster_ptr,
            Ordering::Relaxed,
            Ordering::Relaxed,
        )
    }

    pub fn get_id(&self) -> Id {
        self.id.clone()
    }
}

impl<Id> Hash for ClusterPointInner<Id>
where
    Id: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<Id> PartialEq<Self> for ClusterPointInner<Id>
where
    Id: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<Id> Eq for ClusterPointInner<Id> where Id: Eq {}

pub type ClusterPoint<Id> = Arc<ClusterPointInner<Id>>;

const COMMITTED: u8 = 1;
const ROLLED_BACK: u8 = 1;

pub struct Cluster<Id> {
    pub points: Vec<ClusterPoint<Id>>,
    pub cluster_id: u32,
    state: AtomicU8,
}

impl<Id> Cluster<Id> {
    pub fn new(cluster_id: u32, cluster_size: usize) -> Self {
        Cluster {
            points: Vec::with_capacity(cluster_size),
            cluster_id: cluster_id,
            state: AtomicU8::new(0),
        }
    }

    /// cluster state machine is simple. There are only 3 possible states and 2 transitions:
    /// NEW -> COMMITTED, NEW -> ROLLED_BACK


    fn commit(&self) -> bool {
        self.state
            .compare_exchange(0, COMMITTED, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    }

    fn rollback(&self) -> bool {
        self.state
            .compare_exchange(0, ROLLED_BACK, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    }

    fn is_rolled_back(&self) -> bool {
        self.state.load(Ordering::Relaxed) == ROLLED_BACK
    }

    fn is_commited(&self) -> bool {
        self.state.load(Ordering::Relaxed) == COMMITTED
    }
}

/// Parallel Clusterer searches clusters using multiple threads
///
///
/// the same cluster from a different point.
///
pub struct Clusterer<T, Id> {
    cluster_id_sequence: AtomicU32,
    phandom: PhantomData<(T, Id)>,
    n_threads: usize,
    min_cluster_size: usize,
}

impl<T, Id> Clusterer<T, Id>
where
    Id: Hash + Eq + Clone + Sync + Send,
    T: MinHashType
{
    pub fn new(min_cluster_size: usize, n_threads: usize) -> Self {
        Clusterer {
            cluster_id_sequence: AtomicU32::new(0),
            phandom: PhantomData,
            n_threads: n_threads,
            min_cluster_size: min_cluster_size,
        }
    }

    pub fn cluster(
        &self,
        points: &mut Vec<ClusterPoint<Id>>,
        query_index: &(dyn QueryIndex<Id = ClusterPoint<Id>> + Sync),
    ) -> Vec<Box<Cluster<Id>>> {
        self.cluster_slice(points.as_mut_slice(), query_index)
    }

    pub fn cluster_par<C: IdContainer<ClusterPoint<Id>>>(
        &self,
        points: &mut Vec<ClusterPoint<Id>>,
        query_index: &MinHashIndex<T, ClusterPoint<Id>, C>,
    ) -> Vec<Box<Cluster<Id>>> {
        let chunk_size = points.len() / self.n_threads;
        let result: Vec<_> = points.par_chunks_mut(chunk_size)
            .map(|chunk| self.cluster_slice(chunk, query_index))
            .collect();
        result.into_iter().flat_map(|v| v.into_iter()).collect()
    }

    fn sort_by_most_number_of_similar_points(
        &self,
        points: &mut [ClusterPoint<Id>],
        query_index: &(dyn QueryIndex<Id = ClusterPoint<Id>>),
    ) {
        let point_count: HashMap<Id, usize> = points.iter()
            .map(|point| (point.get_id(), query_index.query(&point).len()))
            .collect();

        points.sort_by(|p1, p2| {
            let count1: &usize = point_count.get(&p1.get_id()).unwrap();
            let count2: &usize = point_count.get(&p2.get_id()).unwrap();
            count2.cmp(&count1)
        });
    }

    fn cluster_slice(
        &self,
        points: &mut [ClusterPoint<Id>],
        query_index: &dyn QueryIndex<Id = ClusterPoint<Id>>,
    ) -> Vec<Box<Cluster<Id>>> {
        let mut clusters = Vec::new();
        self.sort_by_most_number_of_similar_points(points, query_index);
        for point in points {
            if point.cluster.load(Ordering::Relaxed) != null_mut() {
                continue;
            }
            let mut similar_points: Vec<_> = query_index.query(&point).into_iter()
                .filter(|point| !point.is_cluster_assigned())
                .collect();

            if similar_points.len() >= self.min_cluster_size {
                let mut similar_points_cloned =
                    similar_points.into_iter().map(|p| p.clone()).collect();
                match self.try_create_cluster(&mut similar_points_cloned) {
                    Some(cluster) => {
                        clusters.push(cluster);
                    }
                    None => {}
                }
            }
        }
        clusters
    }

    /// Creates a new cluster given the points.
    /// During parallel clustering two threads may attempt to create a cluster from
    /// similar points.
    fn try_create_cluster(&self, points: &mut Vec<ClusterPoint<Id>>) -> Option<Box<Cluster<Id>>> {
        let cluster_id = self.cluster_id_sequence.fetch_add(1, Ordering::Relaxed) + 1;
        let mut cluster: Box<Cluster<Id>> =
            Box::new(Cluster::new(cluster_id, self.min_cluster_size));
        let cluster_const_ptr: *const Cluster<Id> = &*cluster;
        let cluster_ptr = cluster_const_ptr as *mut Cluster<Id>;

        for mut i in 0..points.len() {
            // Another thread may rolled us back
            if cluster.is_rolled_back() {
                self.rollback_cluster_points(&mut cluster, cluster_ptr);
            }
            let point = &points[i];
            match point.assign_cluster(cluster_ptr) {
                Ok(_) => {
                    cluster.points.push(point.clone());
                }
                Err(conflict_ptr) => {
                    // This point has already been assigned a cluster by another thread,
                    // but if the conflict cluster has not been commited we still can win this point
                    let conflict_cluster: &mut Cluster<Id> = unsafe { &mut *conflict_ptr };
                    if conflict_cluster.is_commited() {
                        cluster.rollback();
                        self.rollback_cluster_points(&mut cluster, cluster_ptr);
                        return None;
                    } else if conflict_cluster.is_rolled_back() {
                        i = i - 1; // go back and retry
                    } else if (cluster_id > conflict_cluster.cluster_id) {
                        // We loose because conflict_cluster was first
                        cluster.rollback();
                        self.rollback_cluster_points(&mut cluster, cluster_ptr);
                        return None;
                    } else {
                        if conflict_cluster.rollback() {
                            cluster.points.push(point.clone());
                        } else {
                            // We are too late
                            cluster.rollback();
                            self.rollback_cluster_points(&mut cluster, cluster_ptr);
                            return None;
                        }
                    }
                }
            }
        }
        if !cluster.commit() {
            self.rollback_cluster_points(&mut cluster, cluster_ptr);
            return None;
        }
        Some(cluster)
    }

    fn rollback_cluster_points(
        &self,
        cluster: &mut Box<Cluster<Id>>,
        cluster_ptr: *mut Cluster<Id>,
    ) {
        for point in cluster.points.iter_mut() {
            point.cluster.compare_exchange(
                cluster_ptr,
                null_mut(),
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
        }
    }
}
