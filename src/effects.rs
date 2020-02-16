pub trait Effect: Clone + Send {
   fn run(&mut self, clock: f32) -> f32;
}

#[derive(Clone)]
struct EchoEffect {
   memory:   Vec<f32>,
   diffuser: f32,
}

impl EchoEffect {
   pub fn new(diffuser: f32) -> Self {
      return Self {
         memory: vec![0.0; 100],
         diffuser,
      };
   }
}

impl Effect for EchoEffect {
   fn run(&mut self, clock: f32) -> f32 {
      let memory_clock = self.memory.pop().expect("EchoEffect popped all its memory");
      let diffused_memory = memory_clock * self.diffuser;
      self.memory.push(diffused_memory + clock);
      return diffused_memory + clock;
   }
}
