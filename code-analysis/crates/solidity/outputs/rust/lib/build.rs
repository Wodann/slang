use codegen_parser::GrammarParserGeneratorExtensions;
use codegen_schema::Grammar;
use codegen_utils::context::CodegenContext;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    CodegenContext::with_context(|codegen| {
        let grammar_file = codegen
            .repo_root
            .join("code-analysis/crates/solidity/inputs/schema/manifest.yml");

        let output_dir = codegen
            .repo_root
            .join("code-analysis/crates/solidity/outputs/rust/lib/src/generated");

        let grammar = Grammar::from_manifest(codegen, &grammar_file);
        grammar.generate_rust_lib_sources(codegen, &output_dir);
        return Ok(());
    })?;

    return Ok(());
}
