NODE_ADDRESS := "--node-address http://3.140.179.157:7777"
CLIENT_GET_DICTIONARY_ITEM := "casper-client get-dictionary-item " + NODE_ADDRESS
CLIENT_QUERY := "casper-client query-global-state " + NODE_ADDRESS
CLIENT_GET_STATE_ROOT_HASH := "casper-client get-state-root-hash " + NODE_ADDRESS 

CONTRACT_HASH := "hash-0ffbff03f3d8fcbf5aee2adc81fb6682631f5df507e6c124f6599961bed87a3b"

state-root-hash:
    {{CLIENT_GET_STATE_ROOT_HASH}} | jq -r .result.state_root_hash

to-string-key key:
    cd parser && cargo run --release "to-dictionary-item-key-string" {{key}}

repo-contract-key-raw NAME:
    {{CLIENT_GET_DICTIONARY_ITEM}} \
        --state-root-hash `just state-root-hash` \
        --contract-hash {{CONTRACT_HASH}} \
        --dictionary-name storage_repository_contract \
        --dictionary-item-key `just to-string-key {{NAME}}` \
        | jq -r .result.stored_value.CLValue.bytes

repo-contract-key NAME:
    cd parser && cargo r "decode-repo-value" {{NAME}} `just repo-contract-key-raw {{NAME}}`

example-queries:
    @just repo-contract-key default_policing_rate && \
    just repo-contract-key forum_kyc_required
