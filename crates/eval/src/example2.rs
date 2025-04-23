use crate::{example::LanguageServer, examples::ExampleToml};
use collections::BTreeMap;
use futures::future::LocalBoxFuture;
use gpui::SharedString;
use smol::future::FutureExt;
use std::{fs, path::Path, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExampleId(SharedString);

impl<T: Into<SharedString>> From<T> for ExampleId {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug)]
pub struct ExampleMetadata {
    pub id: ExampleId,
    pub url: String,
    pub revision: String,
    pub language_server: Option<LanguageServer>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AssertionId(SharedString);

impl<T: Into<SharedString>> From<T> for AssertionId {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Default)]
pub struct ExampleSetup {
    assertions: BTreeMap<AssertionId, Vec<Assertion>>,
}

impl ExampleSetup {
    fn assertion(&mut self, id: impl Into<SharedString>) -> AssertionId {
        todo!()
    }
}

struct Assertion {
    condition: bool,
    message: SharedString,
}

struct ExampleRun {
    metadata: ExampleMetadata,
    example: Option<Arc<dyn ErasedExample>>,
    assertions: BTreeMap<AssertionId, Vec<Assertion>>,
}

impl ExampleRun {
    pub fn assert(
        &mut self,
        assertion_id: &AssertionId,
        condition: bool,
        message: impl Into<SharedString>,
    ) {
        self.assertions
            .get_mut(&assertion_id)
            .expect("assertion not found")
            .push(Assertion {
                condition,
                message: message.into(),
            });
    }

    pub fn evaluate(&mut self) {
        let example = self.example.take().unwrap();
        example.evaluate(self);
        self.example = Some(example);
    }
}

trait Example: 'static {
    fn metadata() -> ExampleMetadata
    where
        Self: Sized;
    fn new(cx: &mut ExampleSetup) -> Self
    where
        Self: Sized;
    async fn evaluate(&self, cx: &mut ExampleRun) -> anyhow::Result<()>;
}

trait ErasedExample {
    fn evaluate<'a>(&'a self, cx: &'a mut ExampleRun) -> LocalBoxFuture<'a, anyhow::Result<()>>;
}

struct Erased<T>(T);

impl<T: Example> ErasedExample for Erased<T> {
    fn evaluate<'a>(&'a self, cx: &'a mut ExampleRun) -> LocalBoxFuture<'a, anyhow::Result<()>> {
        self.0.evaluate(cx).boxed_local()
    }
}

struct MyExample {
    my_assertion: AssertionId,
}

impl Example for MyExample {
    fn metadata() -> ExampleMetadata
    where
        Self: Sized,
    {
        ExampleMetadata {
            id: "my_example".into(),
            url: "".into(),
            revision: "".into(),
            language_server: None,
        }
    }

    fn new(cx: &mut ExampleSetup) -> Self
    where
        Self: Sized,
    {
        Self {
            my_assertion: cx.assertion("doesnt_do_stupid_thing"),
        }
    }

    async fn evaluate(&self, cx: &mut ExampleRun) -> anyhow::Result<()> {
        cx.assert(&self.my_assertion, true, "true was false");
        Ok(())
    }
}

struct TomlExample {
    diff_assertions: BTreeMap<AssertionId, SharedString>,
    thread_assertions: BTreeMap<AssertionId, SharedString>,
}

impl ErasedExample for TomlExample {
    fn evaluate<'a>(&'a self, cx: &'a mut ExampleRun) -> LocalBoxFuture<'a, anyhow::Result<()>> {
        // let sample = cx.sample().await?;

        // for (assertion_id, condition) in self.diff_assertions.iter() {
        //     let prompt = diff_prompt.render(diff, condition);
        //     cx.judge(assertion_id, prompt).await?;
        // }

        // for (assertion_id, condition) in self.diff_assertions.iter() {
        //     cx.judge(assertion_id, condition).await?;
        // }

        todo!()
    }
}

#[derive(Default)]
struct EvalSuite {
    example_builders: BTreeMap<ExampleId, Box<dyn Fn() -> ExampleRun>>,
}

impl EvalSuite {
    pub fn examples_to_run(&self, trials: usize) -> BTreeMap<ExampleId, Vec<ExampleRun>> {
        let mut examples = BTreeMap::new();
        for (example_id, builder) in &self.example_builders {
            examples.insert(
                example_id.clone(),
                (0..trials).map(|_| (builder)()).collect(),
            );
        }
        examples
    }

    pub fn register<T: Example>(&mut self) {
        let metadata = T::metadata();
        self.example_builders.insert(
            metadata.id.clone(),
            Box::new(move || {
                let mut cx = ExampleSetup::default();
                let example = T::new(&mut cx);
                ExampleRun {
                    metadata: metadata.clone(),
                    example: Some(Arc::new(Erased(example))),
                    assertions: cx.assertions,
                }
            }),
        );
    }

    pub fn register_toml(&mut self, toml_abs_path: &Path) {
        let name = toml_abs_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let id = ExampleId(name.into());
        let base: ExampleToml =
            toml::from_str(&fs::read_to_string(&toml_abs_path).unwrap()).unwrap();

        let language_server = if base.require_lsp {
            Some(crate::example::LanguageServer {
                file_extension: base
                    .language_extension
                    .expect("Language extension is required when require_lsp = true"),
                allow_preexisting_diagnostics: base.allow_preexisting_diagnostics,
            })
        } else {
            None
        };

        let metadata = ExampleMetadata {
            id: id.clone(),
            url: base.url,
            revision: base.revision,
            language_server,
        };

        self.example_builders.insert(
            id,
            Box::new(move || {
                let mut cx = ExampleSetup::default();
                let mut diff_assertions = BTreeMap::new();
                let mut thread_assertions = BTreeMap::new();
                for (assertion_key, condition) in base.diff_assertions.iter() {
                    let assertion_id = cx.assertion(assertion_key);
                    diff_assertions.insert(assertion_id, condition.into());
                }
                for (assertion_key, condition) in base.thread_assertions.iter() {
                    let assertion_id = cx.assertion(assertion_key);
                    thread_assertions.insert(assertion_id.into(), condition.into());
                }
                ExampleRun {
                    metadata: metadata.clone(),
                    example: Some(Arc::new(TomlExample {
                        diff_assertions,
                        thread_assertions,
                    })),
                    assertions: cx.assertions,
                }
            }),
        );
    }
}

fn main() {
    // let mut suite = EvalSuite::default();
    // suite.register::<MyExample>();
    // for path in list_files(..) {
    //     suite.register_static()
    // }

    // for (example_id, runs) in suite.examples_to_run(2) {}
}
