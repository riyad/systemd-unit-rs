
pub(crate) struct PodmanCommand<'a> {
    args: Vec<&'a str>,
}

impl<'a> PodmanCommand<'a> {
    fn _new() -> Self {
        PodmanCommand {
            args: Vec::with_capacity(10),
        }
    }

    pub(crate) fn add(&mut self, arg: &'a str) {
        self.args.push(arg);
    }

    pub(crate) fn add_slice(&mut self, args: &'a [&str])
    {
        self.args.append(args.to_vec().as_mut())
    }


    pub(crate) fn add_vec(&mut self, args: &'a Vec<String>)
    {
        for arg in args.as_slice() {
            self.args.push(arg)
        }
    }

    pub(crate) fn new_command(command: &'a str) -> Self {
        let mut podman = Self::_new();

        podman.args.push("/usr/bin/podman");
        podman.args.push(command);

        podman
    }

    pub(crate) fn to_escaped_string(&self) -> String {
        shlex::join(self.args.clone())
    }
}