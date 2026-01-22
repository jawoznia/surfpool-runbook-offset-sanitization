# Reproduction of failed sanitization of accounts offsets

Build program and start surfpool (deployment script should be auto run)

```bash
anchor keys list && anchor keys sync && anchor build && surfpool start --watch
```

then in separate terminal

```bash
surfpool run setup -u
```

This should produce

```
x Failed: Failed to broadcast transaction
! Runbook execution aborted
! error at runbooks/setup/main.tx:57:1
! 	error: unable to send and confirm transaction (RPC response error -32002: Transaction simulation failed: Transaction failed to sanitize accounts offsets correctly; )
x Failed - error: unable to send and confirm transaction (RPC response error -32002: Transaction simulation failed: Transaction failed to sanitize accounts offsets correctly; )
```

## Offchain call

To verify that the `add_asset_pair` can be called outside of surfpool runbooks, do:

```bash
cd chain_caller && cargo run
```
