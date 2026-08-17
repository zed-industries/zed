use std::cell::{Cell, RefCell};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::rc::Rc;
use crate::clustering::QueryIndex;

/// Implementation of a single threaded clustering algorithm


#[derive(Debug, Clone)]
pub struct ClusterPointInner<Id> {
    pub id: Id,
    pub cluster: Cell<Option<usize>>
}

unsafe impl<Id> Send for ClusterPointInner<Id> {}
unsafe impl<Id> Sync for ClusterPointInner<Id> {}


impl<Id> ClusterPointInner<Id> {

    pub fn new(id: Id)   -> Self {
        ClusterPointInner {
            id: id,
            cluster: Cell::new(None)
        }
    }

    pub fn assign_cluster(&self, cluster_id: usize) {
        self.cluster.set(Some(cluster_id));
    }
}

impl<Id> Hash for ClusterPointInner<Id>
where Id: Hash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<Id> PartialEq<Self> for ClusterPointInner<Id> where Id: Eq {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
impl<Id> Eq for ClusterPointInner<Id> where Id: Eq  {}


/// the Id of every item in the index must be
pub type ClusterPoint<Id> = Rc<ClusterPointInner<Id>>;


pub struct Cluster<Id> {
    pub points: Vec<ClusterPoint<Id>>,
    pub cluster_id: usize,
}

impl<Id> Cluster<Id> {
    pub fn new(cluster_id: usize) -> Self {
        Cluster {
            cluster_id: cluster_id,
            points: Vec::new()
        }
    }
}

pub struct Clusterer<Id> {
    cluster_id_sequence: usize,
    phandom: PhantomData<Id>,
    min_cluster_size: usize,
}

impl<Id> Clusterer<Id>
    where Id: Hash + Eq + Clone + Display {
    pub fn new(min_cluster_size: usize) -> Self {
        Clusterer {
            cluster_id_sequence: 0,
            phandom: PhantomData,
            min_cluster_size: min_cluster_size
        }
    }

    fn new_cluster_id(&mut self) -> usize {
        self.cluster_id_sequence += 1;
        self.cluster_id_sequence
    }

    pub fn cluster(&mut self,
                   points: &mut Vec<ClusterPoint<Id>>,
                   query_index: & (dyn QueryIndex<Id=ClusterPoint<Id>> )) -> Vec<Box<Cluster<Id>>> {
        let mut clusters = Vec::new();
        let mut cluster_id_seq = self.cluster_id_sequence;
        for point in points {
            if point.cluster.get().is_some() {
                continue;
            }

            let similar_points: Vec<_> = query_index.query(point).into_iter()
                .filter(|point| point.cluster.get().is_none())
                .map(|p| p.clone()).collect();
            if similar_points.len() > self.min_cluster_size {
                cluster_id_seq += 1;
                let cluster = self.create_new_cluster(similar_points, cluster_id_seq);
                clusters.push(cluster);
            }
        }
        clusters
    }

    fn create_new_cluster(&mut self, mut points: Vec<ClusterPoint<Id>>, cluster_id: usize) -> Box<Cluster<Id>> {
        let mut cluster = Box::new(Cluster::new(cluster_id));
        for mut point in points.into_iter() {
            point.assign_cluster(cluster_id);
            cluster.points.push(point);
        }
        cluster
    }
}
