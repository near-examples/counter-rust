describe('Token', function () {
  let near;
  let contract;
  let accountId;

  beforeAll(async function () {
    console.log('nearConfig', nearConfig);
    near = await nearlib.connect(nearConfig);
    accountId = nearConfig.contractName;
    contract = await near.loadContract(nearConfig.contractName, {
      viewMethods: ['get_num'],
      changeMethods: ['increment', 'decrement', 'reset'],
      sender: accountId
    });
  });

  describe('counter', function () {
    it('can be incremented', async function () {
      const startCounter = await contract.get_num();
      await contract.increment();
      const endCounter = await contract.get_num();
      expect(endCounter).toEqual(startCounter + 1);
    });
    it('can be decremented', async function () {
      await contract.increment();
      const startCounter = await contract.get_num();
      await contract.decrement();
      const endCounter = await contract.get_num();
      expect(endCounter).toEqual(startCounter - 1);
    });
    it('can be reset', async function () {
      await contract.increment();
      const startCounter = await contract.get_num();
      await contract.reset();
      const endCounter = await contract.get_num();
      expect(endCounter).toEqual(0);
    });
  });
});