import { Worker, NEAR, NearAccount } from "near-workspaces";
import anyTest, { TestFn } from "ava";

const test = anyTest as TestFn<{
  worker: Worker;
  accounts: Record<string, NearAccount>;
}>;

test.beforeEach(async (t) => {
  // Init the worker and start a Sandbox server
  const worker = await Worker.init();
  const root = worker.rootAccount;

  // deploy contract
  const contract = await root.createAndDeploy(
    root.getSubAccount("rust-counter").accountId,
    "./target/res/counter_contract.wasm",
    { initialBalance: NEAR.parse("30 N").toJSON() }
  );

  // some test accounts
  const alice = await root.createSubAccount("alice", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });
  const bob = await root.createSubAccount("bob", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });
  const charlie = await root.createSubAccount("charlie", {
    initialBalance: NEAR.parse("30 N").toJSON(),
  });

  // Save state for test runs, it is unique for each test
  t.context.worker = worker;
  t.context.accounts = { root, contract, alice, bob, charlie };
});

test.afterEach(async (t) => {
  // Stop Sandbox server
  await t.context.worker.tearDown().catch((error) => {
    console.log("Failed to stop the Sandbox:", error);
  });
});

test("can be incremented", async (t) => {
  const { root, contract } = t.context.accounts;
  const startCounter: number = await contract.view("get_num", {});
  await root.call(contract, "increment", {});
  const endCounter = await contract.view("get_num", {});
  t.is(endCounter, startCounter + 1);
});

test("can be decremented", async (t) => {
  const { root, contract } = t.context.accounts;
  await root.call(contract, "increment", {});
  const startCounter: number = await contract.view("get_num", {});
  await root.call(contract, "decrement", {});
  const endCounter = await contract.view("get_num", {});
  t.is(endCounter, startCounter - 1);
});

test("can be reset", async (t) => {
  const { root, contract } = t.context.accounts;
  await root.call(contract, "increment", {});
  await root.call(contract, "increment", {});
  await root.call(contract, "reset", {});
  const endCounter = await contract.view("get_num", {});
  t.is(endCounter, 0);
});
