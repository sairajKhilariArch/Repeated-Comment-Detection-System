# Social Media Repeated Comment Detection System

A high-performance, concurrent comment monitoring and spam detection engine written in **Rust**. This system ingests a stream of user comments from an external dataset, normalizes text, computes Jaccard similarity for near-duplicate detection, and flags spam patterns using sliding-window history buffers.

**Developed as a tiny systems project by Group 4.**

## Project Team — Group 4

| Name | Roll Number |
|---|---|
| Sandip Bhuriya | 26PG110058 |
| Sairaj Khilari | 26PG110059 |
| Bhavdeep Gohil | 26PG110065 |

---

## 🚀 Key Features

- **Concurrent Stream Simulation**: Uses Rust’s multi-producer, single-consumer (`mpsc`) channel architecture to simulate a real-time comment feed stream with background threading.
- **Robust Text Normalization**: Strips punctuation, handles case insensitivity, and normalizes whitespaces prior to evaluation.
- **Exact & Near-Duplicate Detection**: Evaluates incoming comments against recent history using token-based Jaccard similarity:
  $$J(A, B) = \\frac{|A \\cap B|}{|A \\cup B|}$$
- **Spam Heuristics**: Flags malicious payloads, excessive character repetition, and known bot patterns.
- **External File Ingestion**: Reads comment records from the companion `comments.txt` data file.

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

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

### 1. Clone the repository

```bash
git clone https://github.com/sairajKhilariArch/Repeated-Comment-Detection-System.git
cd Repeated-Comment-Detection-System
```

### 2. Run the project

```bash
cargo run
```

### 3. Run unit tests

```bash
cargo test
```

---

## 📊 Sample Output Log

```text
==================================================
 SOCIAL MEDIA REPEATED COMMENT DETECTION SYSTEM
 Group 4: Sandip Bhuriya, Sairaj Khilari, Bhavdeep Gohil
 Storage Source: External File (comments.txt)
==================================================
[Stream] Comment ID: 1      | Post: video_01  | Author: TechGeek99 | [UNIQUE] Clean comment approved.
[Stream] Comment ID: 2      | Post: video_01  | Author: CodeNovice | [EXACT DUPLICATE] Matches ID: Some("1")
[Stream] Comment ID: 3      | Post: video_01  | Author: SpamBot2024 | [SPAM DETECTED] Reason: Excessive repeated characters/spam
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
