use crate::p2::WasiCtxView;
use crate::p2::bindings::cli::environment;

impl environment::Host for WasiCtxView<'_> {
    fn get_environment(&mut self) -> anyhow::Result<Vec<(String, String)>> {
        Ok(self.ctx.cli.environment.clone())
    }
    fn get_arguments(&mut self) -> anyhow::Result<Vec<String>> {
        Ok(self.ctx.cli.arguments.clone())
    }
    fn initial_cwd(&mut self) -> anyhow::Result<Option<String>> {
        // FIXME: expose cwd in builder and save in ctx
        Ok(None)
    }
}
