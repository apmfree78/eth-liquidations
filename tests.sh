cargo test --test check_aave_events
cargo test --test price_feed
cargo test --test token_price
cargo test --test token_price_hash
cargo test --test token_user_mapping
cargo test --test update_user
cargo test --test validate_tokens
cargo test --test health_factor

:%s/Result<\([^,]*\), Box<dyn std::error::Error\s+\sSend\s+\sSync>>/Result<\1>
