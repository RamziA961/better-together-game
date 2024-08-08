mkdir -p "./src/grpc-client" && 
rm -rf "./src/grpc-client/*.proto" && 
npx protoc --ts_out "./src/grpc-client" --proto_path "../proto" updates.proto

#--ts_opt=unary_rpc_promise=true 
