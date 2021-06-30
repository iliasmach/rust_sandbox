use actix::prelude::*;
use actix::dev::MessageResponse;

#[derive(Debug, Clone)]
pub struct BaseSignal {
    data: Vec<u8>,
}

impl Message for BaseSignal { type Result = (); }

pub struct GetMessages {}

impl Message for GetMessages { type Result = Vec<BaseSignal>; }

pub trait Sender {}

pub struct SenderImpl {
    //receiver: Recipient<BaseSignal>
}

pub trait Recv {}

pub struct RecvA {

}

impl Recv for RecvA {}
impl Recv for RecvB {}

pub struct RecvB {

}

impl Actor for SenderImpl {
    type Context = Context<Self>;
}

impl Actor for RecvA {
    type Context = Context<Self>;
}
impl Actor for RecvB {
    type Context = Context<Self>;
}

pub struct Exchange {
    pub sender: Recipient<GetMessages>,
    pub receivers: Vec<Recipient<BaseSignal>>
}

impl Handler<GetMessages> for SenderImpl {
    type Result = Vec<BaseSignal>;

    fn handle(&mut self, msg: GetMessages, ctx: &mut Context<Self>) -> Self::Result {
        vec![BaseSignal { data: String::from("Привет!").into_bytes() }]
    }
}

impl Handler<BaseSignal> for RecvA {
    type Result = ();

    fn handle(&mut self, msg: BaseSignal, ctx: &mut Context<Self>) -> Self::Result {
        let a = String::from_utf8(msg.data).unwrap();
        println!("{} from RecvA", a)
    }
}

impl Handler<BaseSignal> for RecvB {
    type Result = ();

    fn handle(&mut self, msg: BaseSignal, ctx: &mut Context<Self>) -> Self::Result {
        let a = String::from_utf8(msg.data).unwrap();
        println!("{} from RecvB", a)
    }
}

#[actix_rt::main]
async fn main() {
    let source = SenderImpl::start(SenderImpl {}).recipient::<GetMessages>();
    let recv_a = RecvA::start(RecvA {}).recipient::<BaseSignal>();
    let recv_b = RecvB::start(RecvB{}).recipient::<BaseSignal>();

    let exchange = Exchange { sender: source, receivers: vec![recv_a, recv_b] };

    let messages = exchange.sender.send(GetMessages {}).await.unwrap();

    for message in &messages {
        for receiver in &exchange.receivers {
            receiver.send(message.clone()).await.unwrap()
        }
    }
}
