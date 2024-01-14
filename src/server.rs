//NOTE: most code is copied from here: https://github.com/actix/examples/tree/master/websockets/chat

use actix::prelude::*;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::{HashMap, HashSet};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// `ChatServer` manages chat rooms and responsible for coordinating chat session.
///
/// Implementation is very naive.
#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    room: HashSet<usize>,
    rng: ThreadRng,
}
/// Send message to specific room
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    /// Id of the client session
    pub id: usize,
    /// Peer message
    pub msg: String,
}

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub username: String,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub username: String,
    pub id: usize,
}
impl ChatServer {
    fn new() -> Self {
        ChatServer {
            sessions: HashMap::new(),
            room: HashSet::new(),
            rng: rand::thread_rng(),
        }
    }
    fn send_message(&self, message: &str, skip_id: usize) {
        for id in &self.room {
            if *id != skip_id {
                if let Some(addr) = self.sessions.get(&id) {
                    addr.do_send(Message(message.to_owned()))
                }
            }
        }
    }
}

/// Make actor from `ChatServer`
impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("{} joined", msg.username);

        // notify all users in same room
        self.send_message(&format!("{} joined", msg.username), 0);

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);

        // join session
        self.room.insert(id);

        // send id back
        id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("{} disconnected", msg.username);

        // remove address
        if self.sessions.remove(&msg.id).is_some() {
            // remove session from all rooms
            self.sessions.remove(&msg.id);
        }
        // send message to other users
        self.send_message(&format!("{} disconnected", msg.username), 0);
    }
}

/// Handler for Message message.
impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        self.send_message(msg.msg.as_str(), msg.id);
    }
}
