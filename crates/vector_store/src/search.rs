trait VectorSearch {
    // Given a query vector, and a limit to return
    // Return a vector of id, distance tuples.
    fn top_k_search(&self, vec: &Vec<f32>) -> Vec<(usize, f32)>;
}
