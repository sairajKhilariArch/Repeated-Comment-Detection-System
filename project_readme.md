# Social Media Repeated Comment Detection System

A high-performance, concurrent comment monitoring and spam detection engine written in **Rust**. This system ingests a real-time stream of user comments from an external dataset, normalizes text, computes Jaccard similarity for near-duplicate detection, and flags spam patterns using sliding-window history buffers.

Developed as a tiny systems project by **Group 4**.

---

## 🚀 Key Features

- **Concurrent Stream Simulation**: Uses Rust’s multi-producer, single-consumer (`mpsc`) channel architecture to simulate a real-time comment feed stream with background threading.
- **Robust Text Normalization**: Strips punctuation, handles case insensitivity, and normalizes whitespaces prior to evaluation.
- **Exact & Near-Duplicate Detection**: Evaluates incoming comments against recent history using token-based Jaccard similarity:
  $$J(A, B) = \frac{|A \cap B|}{|A \cup B|}$$
- **Spam Heuristics**: Flags malicious payloads, excessive character repetition, and known bot patterns.
- **External File Ingestion**: Strictly decouples storage by reading from a companion `comments.txt` data file.

---

## 📂 Project Structure

```text
repeated_comment_detector/
├── Cargo.toml
├── comments.txt          # External dataset containing comment records
└── src/
    └── main.rs           # Core engine, normalization, similarity logic, and tests
```

---

## ⚙️ Getting Started & Installation

### Prerequisites
Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

### 1. Clone or Initialize the Project
Create a new Cargo binary project or navigate to your workspace directory:
```bash
cargo new repeated_comment_detector
cd repeated_comment_detector
```

### 2. Add the Code Files
- Replace the contents of `src/main.rs` with the provided detection engine code.
- Place your `comments.txt` dataset file in the root directory alongside `Cargo.toml`.

### 3. Run the Simulation
Execute the streaming simulation using Cargo:
```bash
cargo run
```

### 4. Run Unit Tests
To verify normalization, similarity scoring, and detector logic, run the built-in test suite:
```bash
cargo test
```

---

## 📊 Sample Output Log

```text
==================================================
 SOCIAL MEDIA REPEATED COMMENT DETECTION SYSTEM 
 Group 4: Sandipkumar, Sairaj, Bhavdeep           
 Storage Source: External File (comments.txt)     
==================================================
[Stream] Comment ID: 1      | Post: video_01  | Author: TechGeek99 | ✅ [UNIQUE] Clean comment approved.
[Stream] Comment ID: 2      | Post: video_01  | Author: CodeNovice | 🚨 [EXACT DUPLICATE] Matches ID: Some("1")
[Stream] Comment ID: 3      | Post: video_01  | Author: SpamBot2024 | 🚫 [SPAM DETECTED] Reason: Excessive repeated characters/spam
==================================================
 DETECTION AUDIT SUMMARY 
==================================================
Total Comments Processed: 100
Total Flagged/Duplicates: 42
Clean Unique Comments:    58
Processing Duration:      15.42s
==================================================
```

---

## 🛡️ License
This project is open-source and developed for educational and demonstration purposes.