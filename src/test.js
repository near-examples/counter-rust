describe('Token', function () {
  let near;
  let contract;
  let accountId;

  jasmine.DEFAULT_TIMEOUT_INTERVAL = 10000;

  beforeAll(async function () {
    console.log('nearConfig', nearConfig);
    near = await nearlib.connect(nearConfig);
    accountId = nearConfig.contractName;
    contract = await near.loadContract(nearConfig.contractName, {
      viewMethods: ['get_num'],
      changeMethods: ['do_operation', 'reset'],
      sender: accountId
    });
  });

  describe('counter', function () {
    it('can be incremented', async function () {
      const startCounter = await contract.get_num();
      await contract.do_operation({op: '+', value: 1});
      const endCounter = await contract.get_num();
      expect(endCounter).toEqual(startCounter + 1);
    });
    it('can be decremented', async function () {
      await contract.do_operation({op: '+', value: 1});
      const startCounter = await contract.get_num();
      await contract.do_operation({op: '-', value: 1});
      const endCounter = await contract.get_num();
      expect(endCounter).toEqual(startCounter - 1);
    });
    it('can be reset', async function () {
      await contract.do_operation({op: '+', value: 1});
      const startCounter = await contract.get_num();
      await contract.reset();
      const endCounter = await contract.get_num();
      expect(endCounter).toEqual(0);
    });
  });
});