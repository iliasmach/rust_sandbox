
use std::time::Duration;
use actix::{Message, SyncContext, Actor, Handler, Addr, Context, System, Arbiter, SyncArbiter, ResponseFuture};

#[derive(Clone)]
struct Msg;

impl Message for Msg {
    type Result = ();
}

struct Act;

impl Actor for Act {
    type Context = SyncContext<Self>;
}

impl Handler<Msg> for Act {
    type Result = ();

    fn handle(&mut self, msg: Msg, ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("Receive");
    }
}

struct Act2 {
    addr: Addr<Act>,
}

impl Actor for Act2 {
    type Context = Context<Self>;
}

impl Handler<Msg> for Act2 {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: Msg, ctx: &mut Context<Self>) -> Self::Result {
        println!("Receiving");
        let addr = self.addr.clone();
        Box::pin(async move {
            addr.send(msg.clone()).await;
        })
    }
}

fn main() {
    System::new().block_on(async {
        let arbiter = Arbiter::new();
        let addr = Act2::start_in_arbiter(&arbiter.handle(), |cxt| {
            Act2 {
                addr: SyncArbiter::start(4, || {
                    println!("Starting actor");
                    Act
                })
            }
        });

        println!("Start");

        addr.send(Msg).await;

        let j = std::thread::spawn(move || {
            println!("joining");

            std::thread::sleep(Duration::from_secs(4));
            println!("closing")
        });

        j.join();
    });
}
