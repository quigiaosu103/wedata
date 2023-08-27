export const NetworkId = "testnet"

const TestnetWidgets = {
  profileImage: "eugenethedream/widget/ProfileImage",
};

export const Widgets =
  NetworkId === "testnet" ? TestnetWidgets : MainnetWidgets;
