pub struct FuncLens;

impl FuncLens {
    pub fn rank_func_lens(stats: &[(String, usize)]) -> String {
        let mut output = "Wasm funcs ranked by length:\n\n".to_string();

        // Rank func lens
        for (id, (func_name, func_len)) in stats.iter().enumerate() {
            output.push_str(&format!("{}. {}: {} instructions\n\n", id + 1, func_name, func_len));
        }
        
        output
    }
}
