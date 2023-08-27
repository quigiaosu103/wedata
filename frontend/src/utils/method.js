import { providers } from "near-api-js";


const THIRTY_TGAS = "30000000000000";
const NO_DEPOSIT = "0";

export const viewMethod = async function(near, {
    contractId,
    method,
    args = {}
  }){
    
    if (!near.selector) {
        throw new Error(
          "Wallet selector not initialized"
        );
    }
    const { config } = near;
    const provider = new providers.JsonRpcProvider({ url: config.nodeUrl });

    const res = await provider.query({
    request_type: "call_function",
    account_id: contractId,
    method_name: method,
    args_base64: Buffer.from(JSON.stringify(args)).toString("base64"),
    finality: "optimistic",
    });

    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    return JSON.parse(Buffer.from(res.result).toString());
}

export const callMethod = async function(near, {
    contractId,
    method,
    args = {},
    gas = THIRTY_TGAS,
    deposit = NO_DEPOSIT,
  }){
    if (!near.selector) {
        throw new Error(
          "Wallet selector not initialized"
        );
      }
      // Sign a transaction with the "FunctionCall" action
      const wallet = await (await near.selector).wallet();
      if (wallet) {
        console.log("check accountID", near.accountId)
        if(!near.accountId) return
        return await wallet.signAndSendTransaction({
          signerId: near.accountId,
          receiverId: contractId,
          actions: [
            {
              type: "FunctionCall",
              params: {
                methodName: method,
                args,
                gas,
                deposit,
              },
            },
          ],
        });
      }
  }