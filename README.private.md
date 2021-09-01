## 嘉铮的信息

# 使用cumulus进化跨链ping pong测试



## 编译启动中继链

```bash
#编译
git clone -b release-v0.9.8 https://github.com/paritytech/polkadot
cd polkadot
cargo build --release
#导出链配置文件
./target/release/polkadot build-spec --chain=rococo-local --disable-default-bootnode --raw > rococo-local.json
#运行两个节点
./target/release/polkadot --name alice --chain rococo-local --alice -d ./data/alice --ws-external --rpc-external --rpc-cors all  --node-key 0000000000000000000000000000000000000000000000000000000000000001

./target/release/polkadot --name bob --chain rococo-local --bob -d ./data/bob --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```



## 下载cumlus代码

```BASH
git clone -b polkadot-v0.9.8 https://github.com/paritytech/cumulus
```



## 修改代码

文件 polkadot-parachains/rococo/src/lib.rs

修改barrier

pub type Barrier = (

​    TakeWeightCredit,

​    AllowTopLevelPaidExecutionFrom<All<MultiLocation>>,

​    AllowUnpaidExecutionFrom<ParentOrParentsUnitPlurality>,

​    AllowUnpaidExecutionFrom<SpecParachain>,

​    // ^^^ Parent & its unit plurality gets free execution

);

添加match_type

match_type! {

​    pub type SpecParachain: impl Contains<MultiLocation> = {

​        X2(Parent, Parachain(2000)) | X2(Parent, Parachain(2001))

​    };

}



## 编译cumulus版本平行链

```bash
#编译
cd cumulus
cargo build 
#导出genesis state和wasm文件
./target/debug/polkadot-collator export-genesis-wasm > genesis-wasm
./target/debug/polkadot-collator export-genesis-state --parachain-id 2000 > genesis-state-2000
./target/debug/polkadot-collator export-genesis-state --parachain-id 2001 > genesis-state-2001
#启动两条平行链
RUST_LOG=runtime=debug ./target/debug/polkadot-collator -d ./data/alice --collator --alice --force-authoring --port 40557 --ws-port 9951 --parachain-id 2000 --ws-external --rpc-cors all --rpc-methods=unsafe -- --execution wasm --chain ../polkadot/rococo-local.json --port 40558 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp

RUST_LOG=runtime=debug ./target/debug/polkadot-collator -d ./data/bob --collator --bob --force-authoring --port 40777 --ws-port 9971 --parachain-id 2001 --ws-external --rpc-cors all --rpc-methods=unsafe -- --execution wasm --chain ../polkadot/rococo-local.json --port 40778 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```







## 向中继链注册两条平行链

https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/sudo

![image-20210901120051289](/Users/xjz/Library/Application Support/typora-user-images/image-20210901120051289.png)

parasSudoWrapper->sudoScheduleParaInitialize



https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/parachains

过段时间能看到平行链有2000 和 2001



## 建立平行链hrmp通道

https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/sudo

![image-20210901120515626](/Users/xjz/Library/Application Support/typora-user-images/image-20210901120515626.png)

parasSudoWrapper->sudoEstablishHrmpChannel

建立两次，参数：

2000 2001 7 1000

2001 2000 7 1000



## 2001节点发送start消息

https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9971#/sudo

![image-20210901120847771](/Users/xjz/Library/Application Support/typora-user-images/image-20210901120847771.png)

Spambot->start 参数

2000 0x11



## 查看事件发送结果

https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9971#/explorer

https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9951#/explorer 


## 整理后：

## 中继链部分
### 下载并编译中继链
```json
git clone https://github.com/paritytech/polkadot test-relay-chain-rococo
git fetch
cargo build --release
```

### 生成原生 chain spec
./target/release/polkadot build-spec --chain rococo-local --disable-default-bootnode --raw > rococo-local-cfde.json

### 启动 Alice 节点
./target/release/polkadot --chain rococo-local-cfde.json --alice --tmp

### 启动 Bob 节点
./target/release/polkadot --chain rococo-local-cfde.json --bob --tmp --port 30334

## 平行链部分
### 导出链配置文件

./target/release/polkadot build-spec --chain=rococo-local --disable-default-bootnode --raw > rococo-local.json

