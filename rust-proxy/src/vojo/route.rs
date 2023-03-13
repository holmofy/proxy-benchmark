use core::fmt::Debug;
use dyn_clone::DynClone;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
#[typetag::serde(tag = "type")]
pub trait LoadbalancerStrategy: Sync + Send + DynClone {
    fn get_route(&mut self) -> Result<BaseRoute, anyhow::Error>;

    fn get_debug(&self) -> String {
        String::from("debug")
    }
}
dyn_clone::clone_trait_object!(LoadbalancerStrategy);

impl Debug for dyn LoadbalancerStrategy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let routes = self.get_debug().clone();
        write!(f, "Series{{{}}}", routes)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BaseRoute {
    pub endpoint: String,
    #[serde(default = "default_resource")]
    pub weight: i32,
    pub try_file: Option<String>,
}
fn default_resource() -> i32 {
    100
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RandomRoute {
    pub routes: Vec<BaseRoute>,
}
#[typetag::serde]
impl LoadbalancerStrategy for RandomRoute {
    fn get_route(&mut self) -> Result<BaseRoute, anyhow::Error> {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.routes.len());
        let dst = self.routes[index].clone();
        Ok(dst)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PollRoute {
    #[serde(skip_serializing, skip_deserializing)]
    pub current_index: Arc<AtomicUsize>,
    pub routes: Vec<BaseRoute>,
    #[serde(skip_serializing, skip_deserializing)]
    pub lock: Arc<Mutex<i32>>,
}
#[typetag::serde]
impl LoadbalancerStrategy for PollRoute {
    fn get_route(&mut self) -> Result<BaseRoute, anyhow::Error> {
        let older = self.current_index.fetch_add(1, Ordering::SeqCst);
        let len = self.routes.len();
        let current_index = older % len;
        let dst = self.routes[current_index].clone();
        debug!("PollRoute current index:{}", current_index as i32);
        Ok(dst)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WeightRoute {
    #[serde(skip_serializing, skip_deserializing)]
    pub indexs: Arc<RwLock<Vec<AtomicIsize>>>,
    pub routes: Vec<BaseRoute>,
}
impl WeightRoute {}
#[typetag::serde]
impl LoadbalancerStrategy for WeightRoute {
    fn get_route(&mut self) -> Result<BaseRoute, anyhow::Error> {
        let indexs = self.indexs.read().unwrap();
        for (pos, e) in indexs.iter().enumerate() {
            let old_value = e.fetch_sub(1, Ordering::SeqCst);
            if old_value > 0 {
                let res = &self.routes[pos].clone();
                debug!("WeightRoute current index:{}", pos as i32);
                return Ok(res.clone());
            }
        }
        drop(indexs);

        let mut new_lock = self.indexs.write().unwrap();
        let check_alive = new_lock.iter().any(|f| {
            let tt = f.load(Ordering::SeqCst);
            tt.is_positive()
        });
        if !check_alive {
            let s = self
                .routes
                .iter()
                .map(|s| AtomicIsize::from(s.weight as isize))
                .collect::<Vec<AtomicIsize>>();
            *new_lock = s;
        }
        drop(new_lock);

        let indexs = self.indexs.read().unwrap();
        for (pos, e) in indexs.iter().enumerate() {
            let old_value = e.fetch_sub(1, Ordering::SeqCst);
            debug!("the value is :{}", old_value);
            if old_value > 0 {
                let res = &self.routes[pos].clone();
                debug!("WeightRoute current index:{}", pos as i32);
                return Ok(res.clone());
            }
        }
        Err(anyhow!("WeightRoute get route error"))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn get_routes() -> Vec<BaseRoute> {
        vec![
            BaseRoute {
                endpoint: String::from("http://localhost:4444"),
                weight: 100,
                try_file: None,
            },
            BaseRoute {
                endpoint: String::from("http://localhost:5555"),
                weight: 100,
                try_file: None,
            },
            BaseRoute {
                endpoint: String::from("http://localhost:5555"),
                weight: 100,
                try_file: None,
            },
        ]
    }
    #[test]
    fn test_max_value() {
        let atomic = AtomicUsize::new(0);
        let old_value = atomic.fetch_add(1, Ordering::SeqCst);
        println!("{}", old_value);
    }
    #[test]
    fn test_poll_route_successfully() {
        let routes = get_routes();
        let mut poll_rate = PollRoute {
            current_index: Default::default(),
            routes: routes.clone(),
            lock: Default::default(),
        };
        for i in 0..100 {
            let current_route = poll_rate.get_route().unwrap();
            assert_eq!(current_route, routes[i % routes.len()]);
        }
    }
    #[test]
    fn test_random_route_successfully() {
        let routes = get_routes();
        let mut random_rate = RandomRoute {
            routes: routes.clone(),
        };
        for _ in 0..100 {
            random_rate.get_route().unwrap();
        }
    }
    #[test]
    fn test_weight_route_successfully() {
        let routes = get_routes();
        let mut weight_route = WeightRoute {
            indexs: Default::default(),
            routes: routes.clone(),
        };
        for _ in 0..100 {
            let current_route = weight_route.get_route().unwrap();
            assert_eq!(current_route, routes[0]);
        }
        for _ in 0..100 {
            let current_route = weight_route.get_route().unwrap();
            assert_eq!(current_route, routes[1]);
        }
        for _ in 0..100 {
            let current_route = weight_route.get_route().unwrap();
            assert_eq!(current_route, routes[2]);
        }
        for _ in 0..100 {
            let current_route = weight_route.get_route().unwrap();
            assert_eq!(current_route, routes[0]);
        }
    }
}
