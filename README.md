This library provides a generic wrapper struct DefaultSlash<T> designed to simplify the handling and parsing of command arguments in Discord bots using the poise framework. You may use it just like the following example:
```rs
#[command(slash_command)]
pub async fn search(
    ctx: MyPoiseContext<'_>,
    ephemeral: DefaultSlash<bool>,
) -> MyPoiseResult {
    log::debug("{}", ephemeral.0);
    todo!();
}
