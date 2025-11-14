mod cli;

fn run_deployments_command(args: &[&str], commands: &[&str]) -> Vec<String> {
    cli::run_command("deployments", args, commands, None)
}

#[test]
fn can_encrypt() {
    let output = run_deployments_command(&["encrypt"], &["twice kind fence tip hidden tilt action fragile skin nothing glory cousin green tomorrow spring wrist shed math olympic multiply hip blue scout claw\n", "foo\n"]);
    //assert_eq!(output[0], "Current epoch: 2.05");
}
