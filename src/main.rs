use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub author: String,
    pub text: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum DetectionType {
    ExactMatch,
    NearDuplicate { similarity: f64 },
    SpamPattern { reason: String },
    Unique,
}

#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub comment_id: String,
    pub post_id: String,
    pub author: String,
    pub text: String,
    pub detection_type: DetectionType,
    pub matched_with_id: Option<String>,
}

pub struct TextUtils;

impl TextUtils {
    /// Normalizes comment text: lowercase, removes punctuation and extra whitespace.
    pub fn normalize(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }

    /// Computes Jaccard similarity between two texts based on word tokens.
    pub fn jaccard_similarity(text1: &str, text2: &str) -> f64 {
        let norm1 = Self::normalize(text1);
        let norm2 = Self::normalize(text2);

        let set1: HashSet<&str> = norm1.split_whitespace().collect();
        let set2: HashSet<&str> = norm2.split_whitespace().collect();

        if set1.is_empty() && set2.is_empty() {
            return 1.0;
        }

        let intersection = set1.intersection(&set2).count() as f64;
        let union = set1.union(&set2).count() as f64;

        if union == 0.0 {
            0.0
        } else {
            intersection / union
        }
    }

    /// Detects simple spam patterns (e.g., excessive repeated characters).
    pub fn detect_spam_patterns(text: &str) -> Option<String> {
        if text.chars().count() > 10 {
            let mut repeat_count = 0;
            let chars: Vec<char> = text.chars().collect();
            for i in 1..chars.len() {
                if chars[i] == chars[i - 1] {
                    repeat_count += 1;
                }
            }
            if (repeat_count as f64) / (chars.len() as f64) > 0.6 {
                return Some("Excessive repeated characters/spam".to_string());
            }
        }
        None
    }
}

pub struct CommentDetector {
    // Stores recent comments per post_id for sliding window analysis
    recent_comments: HashMap<String, Vec<Comment>>,
    similarity_threshold: f64,
    max_history_per_post: usize,
}

impl CommentDetector {
    pub fn new(similarity_threshold: f64, max_history_per_post: usize) -> Self {
        CommentDetector {
            recent_comments: HashMap::new(),
            similarity_threshold,
            max_history_per_post,
        }
    }

    /// Analyzes an incoming comment against historical comments for the same post.
    pub fn analyze_comment(&mut self, comment: Comment) -> DetectionResult {
        let post_history = self
            .recent_comments
            .entry(comment.post_id.clone())
            .or_default();

        // 1. Check for spam heuristics first
        if let Some(reason) = TextUtils::detect_spam_patterns(&comment.text) {
            let result = DetectionResult {
                comment_id: comment.id.clone(),
                post_id: comment.post_id.clone(),
                author: comment.author.clone(),
                text: comment.text.clone(),
                detection_type: DetectionType::SpamPattern { reason },
                matched_with_id: None,
            };
            self.add_to_history(comment);
            return result;
        }

        let norm_incoming = TextUtils::normalize(&comment.text);

        // 2. Check historical comments for exact or near duplicates
        for historical in post_history.iter().rev() {
            let norm_historical = TextUtils::normalize(&historical.text);

            // Exact match after normalization
            if norm_incoming == norm_historical {
                let result = DetectionResult {
                    comment_id: comment.id.clone(),
                    post_id: comment.post_id.clone(),
                    author: comment.author.clone(),
                    text: comment.text.clone(),
                    detection_type: DetectionType::ExactMatch,
                    matched_with_id: Some(historical.id.clone()),
                };
                self.add_to_history(comment);
                return result;
            }

            // Near duplicate check using Jaccard similarity
            let similarity = TextUtils::jaccard_similarity(&comment.text, &historical.text);
            if similarity >= self.similarity_threshold {
                let result = DetectionResult {
                    comment_id: comment.id.clone(),
                    post_id: comment.post_id.clone(),
                    author: comment.author.clone(),
                    text: comment.text.clone(),
                    detection_type: DetectionType::NearDuplicate { similarity },
                    matched_with_id: Some(historical.id.clone()),
                };
                self.add_to_history(comment);
                return result;
            }
        }

        // 3. Unique comment
        let result = DetectionResult {
            comment_id: comment.id.clone(),
            post_id: comment.post_id.clone(),
            author: comment.author.clone(),
            text: comment.text.clone(),
            detection_type: DetectionType::Unique,
            matched_with_id: None,
        };
        self.add_to_history(comment);
        result
    }

    fn add_to_history(&mut self, comment: Comment) {
        let history = self
            .recent_comments
            .entry(comment.post_id.clone())
            .or_default();
        history.push(comment);
        if history.len() > self.max_history_per_post {
            history.remove(0);
        }
    }
}

pub fn run_comment_stream_simulation() {
    println!("==================================================");
    println!(" SOCIAL MEDIA REPEATED COMMENT DETECTION SYSTEM ");
    println!("==================================================");

    let (tx, rx): (Sender<Comment>, Receiver<Comment>) = mpsc::channel();

    // Spawn comment ingestion producer thread simulating live social media traffic
    thread::spawn(move || {
        let sample_comments = vec![
            (
                "c1",
                "post_101",
                "alice",
                "This is an amazing tutorial! Thanks for sharing.",
            ),
            (
                "c2",
                "post_101",
                "bob",
                "This is an amazing tutorial! Thanks for sharing.",
            ), // Exact duplicate
            (
                "c3",
                "post_101",
                "charlie",
                "Very helpful, thank you so much!",
            ),
            (
                "c4",
                "post_101",
                "david",
                "This is an amazing tutorial! Thanks for sharing!!",
            ), // Near duplicate
            (
                "c5",
                "post_102",
                "eve",
                "Check out my profile for free crypto! $$$",
            ),
            (
                "c6",
                "post_101",
                "bot_user",
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            ), // Spam pattern
            (
                "c7",
                "post_102",
                "frank",
                "Interesting perspective on this topic.",
            ),
            (
                "c8",
                "post_102",
                "grace",
                "Interesting perspective on this topic!",
            ), // Near duplicate
            (
                "c9",
                "post_101",
                "alice",
                "Checking my previous comment update.",
            ),
            (
                "c10",
                "post_102",
                "bot_user2",
                "Check out my profile for free crypto! $$$",
            ), // Exact duplicate spam
        ];

        for (i, (id, post_id, author, text)) in sample_comments.into_iter().enumerate() {
            let comment = Comment {
                id: id.to_string(),
                post_id: post_id.to_string(),
                author: author.to_string(),
                text: text.to_string(),
                timestamp: 1718000000 + i as u64,
            };

            if tx.send(comment).is_err() {
                println!("Receiver dropped.");
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Initialize detector engine with 0.75 similarity threshold
    let mut detector = CommentDetector::new(0.75, 50);
    let mut total_processed = 0;
    let mut flagged_count = 0;

    let start_time = Instant::now();

    // Consume comments from channel
    while let Ok(comment) = rx.recv_timeout(Duration::from_secs(2)) {
        total_processed += 1;
        let result = detector.analyze_comment(comment);

        print!(
            "[Stream] Comment ID: {:<6} | Post: {:<9} | Author: {:<10} | ",
            result.comment_id, result.post_id, result.author
        );

        match &result.detection_type {
            DetectionType::ExactMatch => {
                flagged_count += 1;
                println!(
                    "🚨 [EXACT DUPLICATE] Matches ID: {:?}",
                    result.matched_with_id.unwrap()
                );
            }
            DetectionType::NearDuplicate { similarity } => {
                flagged_count += 1;
                println!(
                    "⚠️  [NEAR DUPLICATE] Similarity: {:.2}% | Matches ID: {:?}",
                    similarity * 100.0,
                    result.matched_with_id.unwrap()
                );
            }
            DetectionType::SpamPattern { reason } => {
                flagged_count += 1;
                println!("🚫 [SPAM DETECTED] Reason: {}", reason);
            }
            DetectionType::Unique => {
                println!("✅ [UNIQUE] Clean comment approved.");
            }
        }
    }

    println!("==================================================");
    println!(" DETECTION AUDIT SUMMARY ");
    println!("==================================================");
    println!("Total Comments Processed: {}", total_processed);
    println!("Total Flagged/Duplicates: {}", flagged_count);
    println!("Clean Unique Comments:    {}", total_processed - flagged_count);
    println!("Processing Duration:      {:?}", start_time.elapsed());
    println!("==================================================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_normalization() {
        let raw = "Hello, World!!!   This is a Test...  ";
        let normalized = TextUtils::normalize(raw);
        assert_eq!(normalized, "hello world this is a test");
    }

    #[test]
    fn test_jaccard_similarity() {
        let t1 = "Great post thanks for sharing";
        let t2 = "Great post! Thanks for sharing!!";
        let sim = TextUtils::jaccard_similarity(t1, t2);
        assert!(sim > 0.8, "Expected high similarity, got {}", sim);
    }

    #[test]
    fn test_detector_exact_match() {
        let mut detector = CommentDetector::new(0.8, 10);
        let c1 = Comment {
            id: "1".into(),
            post_id: "p1".into(),
            author: "alice".into(),
            text: "Awesome writeup!".into(),
            timestamp: 100,
        };
        let c2 = Comment {
            id: "2".into(),
            post_id: "p1".into(),
            author: "bob".into(),
            text: "Awesome writeup!".into(),
            timestamp: 101,
        };

        let r1 = detector.analyze_comment(c1);
        assert!(matches!(r1.detection_type, DetectionType::Unique));

        let r2 = detector.analyze_comment(c2);
        assert!(matches!(r2.detection_type, DetectionType::ExactMatch));
        assert_eq!(r2.matched_with_id, Some("1".into()));
    }
}

fn main() {
    run_comment_stream_simulation();
}