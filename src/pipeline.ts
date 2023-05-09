export type InputPort = {
  name: string,
};

export type OutputPort = {
  name: string,
};

export type NodeData = {
  label: string,
  inputs: InputPort[],
  outputs: OutputPort[],
};
