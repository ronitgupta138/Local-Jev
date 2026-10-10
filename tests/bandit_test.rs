use local_jev::bandit::{BanditLearner, FeedbackOutcome};

#[test]
fn test_bandit_learner_feedback_updates() {
    let learner = BanditLearner::new();
    let ctx = "search_code";
    let choice_a = "grep";
    let choice_b = "find";

    // Record 5 successes for grep
    for _ in 0..5 {
        learner.record_feedback(ctx, choice_a, FeedbackOutcome::Success);
    }

    // Record 5 failures for find
    for _ in 0..5 {
        learner.record_feedback(ctx, choice_b, FeedbackOutcome::Failure);
    }

    let bonus_a = learner.get_preference_bonus(ctx, choice_a);
    let bonus_b = learner.get_preference_bonus(ctx, choice_b);

    // grep must receive a significantly higher preference bonus than find
    assert!(bonus_a > bonus_b);
}
