use moka::future::Cache;
use std::collections::btree_map;
use std::collections::BTreeMap;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug)]
pub struct User {
    user_id: u64, // Needed as key in BTreeMap when executing a recursive Drop of a Session
    name: String,
    friends: Vec<Arc<Mutex<User>>>,
}

impl User {
    pub fn print_friends(&self) {
        print!("User {} has friends ", self.name);
        for f in &self.friends {
            print!("{}, ", f.lock().unwrap().name);
        }
        println!();
    }
}

impl Drop for User {
    fn drop(&mut self) {
        println!("Dropping user {}", self.name);
    }
}

pub struct Session {
    ptr: Option<Arc<Mutex<User>>>,
    sender: std::sync::mpsc::Sender<u64>,
}

impl Drop for Session {
    fn drop(&mut self) {
        let user_id = self.ptr.as_ref().unwrap().lock().unwrap().user_id;
        println!("Dropping session holding a reference to user {}", user_id);
        self.ptr = None; // Must drop Arc before verify Btree!!!
        let _ = self.sender.send(user_id);
    }
}

#[tokio::main]
async fn main() {
    // For a webserver you may want to access the users via their cached session
    // or via their user number, as they can be friends of each other.
    // Using the Drop trait for a Session, orphaned users will get pruned.
    //
    // Create some users.
    let user1 = Arc::new(Mutex::new(User {
        user_id: 1,
        name: String::from("Alice"),
        friends: vec![],
    }));
    let user2 = Arc::new(Mutex::new(User {
        user_id: 2,
        name: String::from("Bob"),
        friends: vec![],
    }));
    // There will be no session of user Charlie, but he will connected as friend.
    let user3 = Arc::new(Mutex::new(User {
        user_id: 3,
        name: String::from("Charlie"),
        friends: vec![],
    }));
    // Connect their friends to them.
    user2.lock().unwrap().friends.push(user1.clone());
    user2.lock().unwrap().friends.push(user3.clone());
    user2.lock().unwrap().print_friends();

    // Store users names in a B-tree by number.
    let mut group_tree = BTreeMap::new();
    group_tree.insert(1, user1.clone());
    group_tree.insert(2, user2.clone());
    // The group_tree MUST consume user3 here, and not a clone, otherwise
    // strong_count() reports that user3 still has another (unused) reference!
    group_tree.insert(3, user3);

    // Create mpsc channel for pruning user-ids in B-tree.
    let (send, recv) = mpsc::channel::<u64>();
    let send_cl = send.clone();
    let group_tree = Arc::new(Mutex::new(group_tree));
    let group_tree_cl = group_tree.clone();
    thread::spawn(move || loop {
        for u in recv.iter() {
            println!(
                "user id {} has strong count: {}",
                u,
                Arc::strong_count(group_tree_cl.lock().unwrap().get(&u).unwrap())
            );
            let mut verify_queue = Vec::new();
            match group_tree_cl.lock().unwrap().entry(u) {
                btree_map::Entry::Occupied(e) if Arc::strong_count(e.get()) < 2 => {
                    let u = e.remove();
                    for f in u.lock().unwrap().friends.iter() {
                        let u = f.lock().unwrap().user_id;
                        verify_queue.push(u);
                    }
                }
                _ => {}
            };
            // drop here:
            if !verify_queue.is_empty() {
                println!("Send users to verification queue: {:?}", verify_queue);
                for i in verify_queue {
                    let _ = send_cl.send(i);
                }
            }
        }
    });

    // Later, we will check the entry count of the session_cache with this time_step
    // interval.
    let time_step = 1; // second

    // Make an artificially small cache and 2.5-second ttl to observe pruning of the tree.
    // Caution: setting ttl to exact integer multiples of the time steps may cause
    // different behavior than you expect, due to rounding or race conditions.
    let ttl_ms = 2500;
    let sessions_cache = Cache::builder()
        .max_capacity(10)
        .time_to_live(Duration::from_millis(ttl_ms))
        .eviction_listener(|key, value: Arc<Mutex<Session>>, cause| {
            println!(
                "Evicted session with key {:08X} of user_id {:?} because {:?}",
                *key,
                value
                    .lock()
                    .unwrap()
                    .ptr
                    .as_ref()
                    .unwrap()
                    .lock()
                    .unwrap()
                    .user_id,
                cause
            )
        })
        .build();
    // To create some simple CRC-32 session keys with Bash do:
    //   for ((i = 1; i < 4 ; i++)); do rhash <(echo "$i")|tail -1; done

    // Alice's session on browser
    let session1 = Session {
        ptr: Some(user1.clone()),
        sender: send.clone(),
    };
    sessions_cache
        .insert(0x6751FC53, Arc::new(Mutex::new(session1)))
        .await;

    // Alice's second session on smartphone
    let session2 = Session {
        ptr: Some(user1),
        sender: send.clone(),
    };
    sessions_cache
        .insert(0x4C7CAF90, Arc::new(Mutex::new(session2)))
        .await;

    // Add also Bob's session
    let session3 = Session {
        ptr: Some(user2),
        sender: send.clone(),
    };
    sessions_cache
        .insert(0x55679ED1, Arc::new(Mutex::new(session3)))
        .await;

    // Show cache content
    for (key, value) in sessions_cache.iter() {
        let session = value.lock().unwrap();
        println!(
            "Found session {:08X} from user_id: {}",
            *key,
            session.ptr.as_ref().unwrap().lock().unwrap().user_id
        );
    }

    println!("Waiting");
    for t in 1..=4 {
        sleep(Duration::from_secs(time_step));
        sessions_cache.get(&0).await;
        sessions_cache.run_pending_tasks().await;
        println!("t = {}, pending: {}", t, sessions_cache.entry_count());
    }
    assert!(group_tree.lock().unwrap().is_empty());
    println!("Exit program.");
}
