use codespan::{Files, FileId};
use nom::lib::std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use exitcode::ExitCode;

/// Possible intermediate representations which can be emitted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Emit {
    /// The Abstract Syntax Tree. [see wikipedia](https://en.wikipedia.org/wiki/Abstract_syntax_tree)
    AbstractSyntaxTree,
}

/// The default target for wright.
pub const DEFAULT_TARGET: Target = Target::Treewalker;

/// Possible backends that can be targeted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Target {
    /// A tree-walking interpreter for wright.
    /// Tree-walking interpreters evaluate the AST in place. These are generally
    /// inefficient. Other targets should be preferred when they become available.
    Treewalker,
}


#[derive(Debug, Clone)]
/// Interpreter instance.
pub struct Wright {
    files: Files<String>,
    handles: Vec<FileId>,
    verbose: bool,
    emits: HashSet<Emit>,
    target: Target,
    open_repl: bool,
    run: bool
}

impl Wright {
    /// Construct a new instance of the Wright interpreter.
    pub fn new() -> Self {
        Wright {
            files: Files::new(),
            handles: Vec::new(),
            verbose: false,
            emits: HashSet::default(),
            target: DEFAULT_TARGET,
            open_repl: false,
            run: false,
        }
    }

    /// Set this interpreters verbosity flag.
    pub fn verbose(&mut self, val: bool) -> &mut Self {
        self.verbose = val;
        self
    }

    /// Set this interpreter's interactive flag (whether this interpreter will
    /// open a REPL session when called).
    pub fn interactive(&mut self, val: bool) -> &mut Self {
        self.open_repl = val;
        self
    }

    /// Set this interpreter's run flag. This interpreter will run the created
    /// executable after compilation.
    pub fn run(&mut self, val: bool) -> &mut Self {
        self.run = val;
        self
    }


    /// Add source to the Wright Interpreter.
    pub fn add_source(
        &mut self,
        name: impl Into<String> + std::fmt::Debug,
        content: impl Into<String>,
    ) -> &mut Self {
        if self.verbose {
            println!("Loading {:?}.", name)
        }
        let handle = self.files.add(name, content.into());
        self.handles.push(handle);
        self
    }

    /// Add several files to this Wright Interpreter.
    pub fn add_files(&mut self, filenames: Vec<&str>) -> std::io::Result<&mut Self> {
        for fname in filenames {
            let mut f = File::open(fname)?;
            let mut source = String::new();
            f.read_to_string(&mut source)?;
            self.add_source(fname, source);
        }
        Ok(self)
    }

    /// Set certain emit flags.
    pub fn set_emit(&mut self, emit: Emit, value: bool) -> &mut Self {
        if value {
            self.emits.insert(emit);
        } else {
            self.emits.remove(&emit);
        }
        self
    }

    /// Set emit flags to true for all flags in given `emits` Vec,
    /// and false for any others.
    pub fn set_emits(&mut self, emits: Vec<Emit>) -> &mut Self {
        emits.iter().for_each(|e| {
            self.emits.insert(*e);
        });
        self
    }

    /// Set the target backend for this wright interpreter.
    pub fn set_target(&mut self, target: Target) -> &mut Self {
        self.target = target;
        self
    }

    /// Calls and consumes this wright interpreter. Returns exitcode.
    pub fn call(self) -> ExitCode {
        
    }
}
