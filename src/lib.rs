mod engine;
#[cfg(test)]
mod tests {
    use crate::engine::{Engine, EngineTestExt};
    use csv::Reader;
    use std::cmp::Ordering;
    use std::collections::{BTreeSet, HashSet};
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::time::Instant;

    #[derive(Eq)]
    struct TestCase {
        fen: String,
        moves: HashSet<String>
    }

    impl PartialEq<Self> for TestCase {
        fn eq(&self, other: &Self) -> bool {
            self.fen == other.fen
        }
    }

    impl PartialOrd<Self> for TestCase {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.fen.cmp(&other.fen))
        }
    }

    impl Ord for TestCase{
        fn cmp(&self, other: &Self) -> Ordering {
            self.fen.cmp(&other.fen)
        }
    }

    #[test]
    fn test_legal_moves() -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = Reader::from_path("test-cases/legal_moves.csv")?;

        let mut hash_map = BTreeSet::new();

        for result in rdr.records() {
            let record = result?;
            let moves: HashSet<String> = record[1]
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|s| s.trim().trim_matches('\''))
                .map(String::from)
                .collect();
            hash_map.insert(TestCase{ fen:record[0].to_string(), moves });
        }

        let mut grand_total = 0;
        let mut grand_correct_total = 0;
        let mut grand_wrong_total = 0;
        let start = Instant::now();
        for test_cases in hash_map.iter() {
            let mut engine = Engine::new_from_fen(&*test_cases.fen);
            let generated_moves = engine.generate_moves();
            // println!("{}", test_cases.fen);
            // println!("{:?}", generated_moves);
            // println!("{:?}", test_cases.moves);

            let mut correct_count = 0;
            let mut wrong_count = 0;
            for mov in generated_moves.iter() {
                match test_cases.moves.get(mov) {
                    Some(_) => {
                        correct_count += 1;
                    }
                    _ => {
                        wrong_count += 1;
                    }
                }
            }
            grand_wrong_total += wrong_count;
            grand_total += test_cases.moves.len();
            grand_correct_total += correct_count;
        }
        let elapsed = start.elapsed();
        let mut file = OpenOptions::new()
            .append(true)
            .create(true) // Creates the file if it doesn't exist
            .open("test-cases/tests.txt")?;

        writeln!(file,"total: {}, correct total: {}, wrong total: {} in {} ms", grand_total, grand_correct_total, grand_wrong_total, elapsed.as_millis())?;
        Ok(())
    }
}