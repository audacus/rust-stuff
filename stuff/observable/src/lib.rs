trait InductionLoopObserver {
    fn update(&self, induction_loop: &InductionLoop);
}

struct InductionLoop<'a> {
    observers: Vec<Box<&'a dyn InductionLoopObserver>>,
    state: bool,
}

impl<'a> InductionLoop<'a> {

    fn getState(&self) -> bool {
        self.state
    }

    fn setState(&mut self, state: bool) {
        self.state = state;
        self.notify();
    }

    fn attach(&mut self, observer: &'a dyn InductionLoopObserver) {
        self.observers.push(Box::new(observer));
    }

    fn detach(&mut self, observer: &dyn InductionLoopObserver) {
        self.observers
            .retain(|x| *x.as_ref() as *const _ != observer as *const _);
    }

    fn notify(&self) {
        for observer in &self.observers {
            observer.update(&self);
        }
    }
}

struct TrafficLight {}

impl InductionLoopObserver for TrafficLight {

    fn update(&self, induction_loop: &InductionLoop) {
        println!("state is: {}", induction_loop.getState());
    }
}
