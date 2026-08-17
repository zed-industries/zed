use super::{CHANNEL_TEST_ITERATIONS, CHANNEL_TEST_SENDERS};

pub fn capacity_iter() -> impl Iterator<Item = usize> {
    (1..6).map(|i| 2usize.pow(i))
}

// TODO: iterators over channel sender/receiver counts, channel sizes
pub struct MessageIter<I> {
    sender: usize,
    iter: I,
}

impl<I> Iterator for MessageIter<I>
where
    I: Iterator<Item = usize>,
{
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|index| Message {
            sender: self.sender,
            index,
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Message {
    sender: usize,
    index: usize,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            sender: 0,
            index: 0,
        }
    }
}

impl Message {
    pub fn new_iter(sender: usize) -> impl Iterator<Item = Message> {
        MessageIter {
            sender,
            iter: (0..CHANNEL_TEST_ITERATIONS),
        }
    }

    pub fn new_multi_sender(sender: usize) -> impl Iterator<Item = Message> {
        let messages = CHANNEL_TEST_ITERATIONS / CHANNEL_TEST_SENDERS;
        MessageIter {
            sender,
            iter: (0..messages),
        }
    }
}

pub struct Channels {
    channels: Vec<Channel>,
}

impl Channels {
    pub fn new(senders: usize) -> Self {
        let mut channels = Vec::with_capacity(senders);
        for i in 0..senders {
            channels.push(Channel::new(i));
        }
        Self { channels }
    }

    pub fn allow_skips(mut self) -> Self {
        for channel in self.channels.iter_mut() {
            channel.allow_skips = true;
        }

        self
    }

    #[track_caller]
    pub fn assert_message(&mut self, message: &Message) {
        self.channels[message.sender].assert_message(message);
    }
}

pub struct Channel {
    sender: usize,
    current_index: usize,
    allow_skips: bool,
}

impl Channel {
    pub fn new(sender: usize) -> Self {
        Self {
            sender,
            current_index: 0,
            allow_skips: false,
        }
    }

    pub fn allow_skips(mut self) -> Self {
        self.allow_skips = true;
        self
    }

    #[track_caller]
    pub fn assert_message(&mut self, message: &Message) {
        if self.allow_skips {
            assert_eq!(self.sender, message.sender);
            if message.index < self.current_index {
                panic!(
                    "for sender {}, expected message >= {}, found {}",
                    self.sender, self.current_index, message.index
                );
            }

            self.current_index = message.index;
            return;
        }

        assert_eq!(
            (self.sender, self.current_index),
            (message.sender, message.index)
        );
        self.current_index += 1;
    }
}
