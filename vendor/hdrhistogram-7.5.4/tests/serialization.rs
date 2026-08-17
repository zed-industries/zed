#[cfg(all(feature = "serialization", test))]
mod tests {
    use hdrhistogram::serialization::{Deserializer, Serializer, V2Serializer};
    use hdrhistogram::Histogram;

    use std::fs::File;
    use std::io::{BufRead, BufReader, Read};
    use std::path::Path;

    #[test]
    fn serialize_no_compression_matches_java_impl() {
        let h = load_histogram_from_num_per_line(Path::new("tests/data/seq-nums.txt"));

        let mut serialized = Vec::new();
        V2Serializer::new().serialize(&h, &mut serialized).unwrap();

        let mut java_serialized = Vec::new();
        File::open("tests/data/seq-nums.hist")
            .unwrap()
            .read_to_end(&mut java_serialized)
            .unwrap();

        assert_eq!(java_serialized, serialized);
    }

    // zlib compression is not identical between the rust and java versions, so can't compare
    // compressed versions byte for byte

    #[test]
    fn deserialize_no_compression_matches_java() {
        let h = load_histogram_from_num_per_line(Path::new("tests/data/seq-nums.txt"));

        let deser_h: Histogram<u64> = Deserializer::new()
            .deserialize(&mut File::open("tests/data/seq-nums.hist").unwrap())
            .unwrap();

        assert_eq!(h, deser_h);
    }

    #[test]
    fn deserialize_compression_matches_java() {
        let h = load_histogram_from_num_per_line(Path::new("tests/data/seq-nums.txt"));

        let deser_h: Histogram<u64> = Deserializer::new()
            .deserialize(&mut File::open("tests/data/seq-nums.histz").unwrap())
            .unwrap();

        assert_eq!(h, deser_h);
    }

    #[test]
    fn total_count_overflow_from_deserialize_saturates() {
        let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();

        // can't go bigger than i64 max because it will be serialized
        h.record_n(1, i64::max_value() as u64).unwrap();
        h.record_n(1000, i64::max_value() as u64).unwrap();
        h.record_n(1000_000, i64::max_value() as u64).unwrap();
        assert_eq!(u64::max_value(), h.len());

        let mut vec = Vec::new();

        V2Serializer::new().serialize(&h, &mut vec).unwrap();
        let deser_h: Histogram<u64> = Deserializer::new()
            .deserialize(&mut vec.as_slice())
            .unwrap();
        assert_eq!(u64::max_value(), deser_h.len());
    }

    fn load_histogram_from_num_per_line(path: &Path) -> Histogram<u64> {
        // max is Java's Long.MAX_VALUE
        let mut h: Histogram<u64> =
            Histogram::new_with_bounds(1, u64::max_value() >> 1, 3).unwrap();
        for num in BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|l| l.unwrap().parse().unwrap())
        {
            h.record(num).unwrap();
        }

        h
    }
}
