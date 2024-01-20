trait Pizza {
    fn cost(&self) -> f64;
}

struct PlainPizza;

impl Pizza for PlainPizza {
    fn cost(&self) -> f64 {
        // base cost
        40.0
    }
}

/// decorator
struct CheeseDecorator {
    pizza: Box<dyn Pizza>,
}

impl Pizza for CheeseDecorator {
    fn cost(&self) -> f64 {
        self.pizza.cost() + 5.0
    }
}

struct MeatDecorator {
    pizza: Box<dyn Pizza>,
}

impl Pizza for MeatDecorator {
    fn cost(&self) -> f64 {
        self.pizza.cost() + 6.7
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cheese_meat_pizza_cost() {
        let plain_pizza = Box::new(PlainPizza);
        let pizza_with_cheese = Box::new(CheeseDecorator { pizza: plain_pizza });
        let pizza_with_cheese_meat = Box::new(MeatDecorator { pizza: pizza_with_cheese });
        assert_eq!(pizza_with_cheese_meat.cost(), 51.7);
    }
}