use local_jev::bandit::{BanditLearner, FeedbackOutcome};
use tempfile::tempdir;

#[test]
fn test_bandit_checkpoint_persistence() {
    let dir = tempdir().expect("Failed to create tempdir");
    let checkpoint_file = dir.path().join("bandit_state.jev");

    let learner1 = BanditLearner::new();
    learner1.record_feedback("build", "cargo build", FeedbackOutcome::Success);
    learner1.record_feedback("build", "cargo build", FeedbackOutcome::Success);
    learner1.record_feedback("build", "npm install", FeedbackOutcome::Failure);

    let bonus1 = learner1.get_preference_bonus("build", "cargo build");
    assert!(bonus1 > 0.0);

    // Save checkpoint
    learner1
        .save_checkpoint(&checkpoint_file)
        .expect("Checkpoint save should succeed");

    assert!(checkpoint_file.exists());

    // Load checkpoint into fresh learner
    let learner2 = BanditLearner::new();
    let loaded = learner2
        .load_checkpoint(&checkpoint_file)
        .expect("Checkpoint load should succeed");

    assert!(loaded >= 2);
    let bonus2 = learner2.get_preference_bonus("build", "cargo build");
    assert!((bonus1 - bonus2).abs() < 1e-4);
}
