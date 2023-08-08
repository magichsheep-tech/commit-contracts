/*
    This test file tests the basic functionality of the quest contract.
    -claim: career aspirant claims a quest
    -submit: career aspirant submits results to a quest
    -grade: quest provider grades a quest
*/

use std::vec;

use gtest::{ Log, Program, System };
use parity_scale_codec::Encode;
use quest_io::{ QuestEvent, QuestAction };
const QUEST_ID: u64 = 1;
const SELF_ID: u64 = 2;
// const NON_EXIST_ID: u64 = 3;

#[test]
fn claim_success() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    let res = program.send(SELF_ID, QuestAction::Claim(String::from("a fake quest id for testing only")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::Claimed);
    assert!(res.contains(&log));
}

// a claimer cannot claim a quest twice
#[test]
fn claim_fail_double_claim() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    program.send(SELF_ID, QuestAction::Claim(String::from("a fake quest id for testing only")));
    let res = program.send(2, QuestAction::Claim(String::from("a fake quest id for testing only")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::ErrorClaimerExists);
    assert!(res.contains(&log));
}

// cannot claim a non exists quest
#[test]
fn claim_fail_non_exist_quest() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    let res = program.send(SELF_ID, QuestAction::Claim(String::from("a non exists quest id")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::UnknownError);
    assert!(res.contains(&log));
}

#[test]
fn submit_success() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    program.send(SELF_ID, QuestAction::Claim(String::from("a fake quest id for testing only")));
    let res = program.send(SELF_ID, QuestAction::Submit(String::from("a fake quest id for testing only"), String::from("link to submission")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::Submitted);
    assert!(res.contains(&log));
}

// only exising claimers can submit to a quest
#[test]
fn submit_fail_non_existing_claimer() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    // submit without claim the quest first will fail
    let res = program.send(SELF_ID, QuestAction::Submit(String::from("a fake quest id for testing only"), String::from("link to submission")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::ErrorSubmitterNotExists);
    assert!(res.contains(&log));
}

// a claimer can only submit to a quest once
#[test]
fn submit_fail_double_submission() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    program.send(SELF_ID, QuestAction::Claim(String::from("a fake quest id for testing only")));
    program.send(SELF_ID, QuestAction::Submit(String::from("a fake quest id for testing only"), String::from("submission")));
    let res = program.send(SELF_ID, QuestAction::Submit(String::from("a fake quest id for testing only"), String::from("link to submission")));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::ErrorAlreadySubmitted);
    assert!(res.contains(&log));
}

// TODO: need to add a test for submitting pass a deadline

#[test]
fn publish_success() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);

    let quest = quest::QuestInfo {
        entity_name: String::from("test quest entity"),
        location: String::from("test quest location"),
        communication_language: vec![String::from("English")],
        communication_channel: String::from("Email"),
        quest_name: String::from("test quest name"),
        skill_badges: vec![String::from("test skill badge")],
        max_claimers: 1,
        description: String::from("test quest description"),
        deadline: 0,
    };

    let encoded_quest: Vec<u8> = quest.encode();

    program.send(SELF_ID, QuestAction::Publish(encoded_quest));
}
/*
#[test]
fn grade_success() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    program.send(SELF_ID, QuestAction::Claim);
    let res = program.send(SELF_ID, QuestAction::Grade(SELF_ID.into(), 100));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::Graded);
    assert!(res.contains(&log));
}

// only quest owner can grade a quest
#[test]
fn grade_fail_not_grader() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    let res = program.send(NON_EXIST_ID, QuestAction::Grade(NON_EXIST_ID.into(), 100));
    let log = Log::builder().dest(NON_EXIST_ID).payload(QuestEvent::ErrorNotQuestOwner);
    assert!(res.contains(&log));
}

#[test]
fn grade_fail_recipient_not_exists() {
    let sys = System::new();
    init_quest(&sys);
    let program = sys.get_program(QUEST_ID);
    let res = program.send(SELF_ID, QuestAction::Grade(NON_EXIST_ID.into(), 100));
    let log = Log::builder().dest(SELF_ID).payload(QuestEvent::ErrorSubmitterNotExists);
    assert!(res.contains(&log));
} */

fn init_quest(sys: &System) {
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(SELF_ID, String::from("Hello Quest Contract!"));
    let log = Log::builder().dest(SELF_ID).payload(String::from("Quest Created!"));
    assert!(res.contains(&log));
    
}