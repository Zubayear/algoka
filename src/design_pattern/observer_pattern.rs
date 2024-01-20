use std::borrow::Cow;
use std::fmt::format;

/// Subject - it has the state that the observers are interested to know about
trait LiveMatch {
    fn register(&mut self, observer: Box<dyn Observer>);
    fn deregister(&mut self, observer: Box<dyn Observer>);
    fn notify(&self, match_info: MatchInfo);
}

/// Concrete subject
struct ConcreteSubject {
    pub observers: Vec<Box<dyn Observer>>,
    pub match_info: Option<MatchInfo>,
}

impl LiveMatch for ConcreteSubject {
    fn register(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer)
    }

    fn deregister(&mut self, observer: Box<dyn Observer>) {
        self.observers.retain(|o| !std::ptr::eq(o.as_ref(), observer.as_ref()))
    }

    fn notify(&self, match_info: MatchInfo) {
        for observer in &self.observers {
            observer.update(&match_info);
        }
    }
}

pub struct GoalInfo {
    pub scored_by: String,
    pub assist_by: Option<String>,
}

pub struct FoulInfo {
    pub fouled_by: String,
    pub card_name: Option<String>,
}

enum MatchInfo {
    Goal(GoalInfo),
    Foul(FoulInfo),
}

trait Observer {
    fn update(&self, match_info: &MatchInfo);
}

/// Concrete observer - will get the notification about match info from subject
pub struct MobileApp;

pub struct WebApp;

impl Observer for MobileApp {
    fn update(&self, match_info: &MatchInfo) {
        assert_eq!(format_match_info(match_info), Cow::Borrowed("Goal scored by Mohammad Salah and assist by Trent."));
    }
}

impl Observer for WebApp {
    fn update(&self, match_info: &MatchInfo) {
        assert_eq!(format_match_info(match_info), Cow::Borrowed("Fouled by Son and yellow card given."));
    }
}

fn format_match_info(match_info: &MatchInfo) -> Cow<'static, str> {
    match match_info {
        MatchInfo::Goal(goal) => {
            let scored_by = Cow::Borrowed(&goal.scored_by);
            let assist_by = goal.assist_by.as_deref().unwrap_or_default();
            Cow::Owned(format!("Goal scored by {} and assist by {}.", scored_by, assist_by))
        },
        MatchInfo::Foul(foul) => {
            let card_name = foul.card_name.as_deref().unwrap_or_default();
            Cow::Owned(format!("Fouled by {} and {} card given.", foul.fouled_by, card_name))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_info() {
        // concrete observer
        let mobile_app = MobileApp;

        // state
        let goal_info = GoalInfo {
            scored_by: "Mohammad Salah".to_string(),
            assist_by: Option::from("Trent".to_string()),
        };

        let match_info_goal = MatchInfo::Goal(goal_info);

        let mut concrete_subject = ConcreteSubject {
            observers: Vec::new(),
            match_info: None,
        };

        // subject register observer
        concrete_subject.register(Box::new(mobile_app));

        // subject notify observer about state change
        concrete_subject.notify(match_info_goal);
    }

    #[test]
    fn test_foul_info() {
        // concrete observer
        let web_app = WebApp;

        // state
        let foul_info = FoulInfo {
            fouled_by: "Son".to_string(),
            card_name: Option::from("yellow".to_string()),
        };

        let match_foul_goal = MatchInfo::Foul(foul_info);

        let mut concrete_subject = ConcreteSubject {
            observers: Vec::new(),
            match_info: None,
        };

        // subject register observer
        concrete_subject.register(Box::new(web_app));

        // subject notify observer about state change
        concrete_subject.notify(match_foul_goal);
    }
}