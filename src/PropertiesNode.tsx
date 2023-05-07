import type { NodeProps } from 'reactflow';

import { Handle, Position } from 'reactflow';
import { css } from '@acab/ecsstatic';

type Input = {
  name: string,
};

type Output = {
  name: string,
};

const LabelCSS = css`
  background: #fff;
  grid-row: 1 / 2;
  grid-column: 1 / 3;
  text-align: left;
  vertical-align: baseline;
  border-radius: 5px 5px 0 0;
`;

const Label: React.FC<{label: string}> = ({ label }) => (
  <div className={LabelCSS}>
    {label}
  </div>
);

const InputPortsCSS = css`
  background: #fff;
  grid-row: 2 / 3;
  grid-column: 1 / 2;
  text-align: left;
  vertical-align: baseline;
  border-radius: 0 0 0 5px;
`;

const PortCSS = css`
  display: inline-block;
  position: relative;
`;


const InputPort: React.FC<Input> = ({name}) => (
  <p>
    <Handle type="target" position={Position.Left} id="in-a" className={PortCSS} />
    {name}
  </p>
)

const InputPorts: React.FC<{inputs: Input[]}> = ({inputs}) => (
  <div className={InputPortsCSS}>
    {inputs.map(input => (<InputPort {...input} />))}
  </div>
);

const OutputPortsCSS = css`
  background: #fff;
  grid-row: 2 / 3;
  grid-column: 2 / 3;
  text-align: right;
  vertical-align: baseline;
  border-radius: 0 0 5px 0;
`;

const OutputPort: React.FC<Output> = ({name}) => (
  <p>
    {name}
    <Handle className={PortCSS} type="source" position={Position.Right} id="out-a" />
  </p>
);

const OutputPorts: React.FC<{outputs: Output[]}> = ({outputs}) => (
  <div className={OutputPortsCSS}>
    {outputs.map(output => (<OutputPort {...output} />))}
  </div>
);

const PropertiesNodeCSS = css`
  background: #000;
  border: 1px solid #000;
  min-width: 300px;
  display: grid;
  grid-template-rows: 30px auto;
  grid-template-columns: 1fr 1fr;
  gap: 1px;
  border-radius: 5px;
`;

export const PropertiesNode: React.FC<NodeProps<{
  label: string,
  inputs: Input[],
  outputs: Output[],
}>> = ({ data: { label, inputs, outputs } }) => (
  <div className={PropertiesNodeCSS}>
    <Label label={label} />
    <InputPorts inputs={inputs} />
    <OutputPorts outputs={outputs} />
  </div>
);
