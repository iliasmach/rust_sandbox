use actix::{Actor, ActorContext, Context, AsyncContext, SyncContext, System, Arbiter};
use std::time::Duration;

struct ServiceA {}

impl Actor for ServiceA {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Started");

        let f = ctx.run_interval(Duration::from_millis(50), |this, ctx| {
            println!("From Later");
        });
    }
}


fn main() {
    System::new().block_on(async {
        let arbiter = Arbiter::new();
        let service = ServiceA::start_in_arbiter(&arbiter.handle(), |ctx| {
            ServiceA {}
        });


        std::thread::sleep(Duration::from_millis(50));
        std::thread::sleep(Duration::from_millis(150));
        std::thread::sleep(Duration::from_millis(1150));
    });
}
