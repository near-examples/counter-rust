import {Workspace} from 'near-workspaces-ava';

const workspace = Workspace.init(async ({root}) => {
  const alice = await root.createAccount('alice');
  const contract = await root.createAndDeploy(
    'rust-counter',
    '../out/main.wasm',
  );
  await alice.call(contract, 'reset', {});
  return {alice, contract};
});


workspace.test('test-increment', async (testava, {contract, root}) => {
  await root.call(contract, 'increment', {});
  testava.is(
    await contract.view('get_num'),
    1,
  );
});

workspace.test('test-decrement', async (testava, {contract, root}) => {
  await root.call(contract, 'decrement', {});
  testava.is(
    await contract.view('get_num'),
    -1,
  );
});

workspace.test('test-increment-and-reset', async (testava, {contract, root}) => {
  await root.call(contract, 'increment', {});
  await root.call(contract, 'reset', {});
  testava.is(
    await contract.view('get_num'),
    0,
  );
});

