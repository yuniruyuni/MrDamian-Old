import type { NodeProps } from 'reactflow';

import { Handle, Position } from 'reactflow';
import { css } from '@acab/ecsstatic';

import { presentation } from '~/go/models';

const LabelCSS = css`
  background: #fff;
  grid-row: 1 / 2;
  grid-column: 1 / 3;
  text-align: left;
  vertical-align: baseline;
  border-radius: 5px 5px 0 0;
`;

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

const Label: React.FC<{ label: string }> = ({ label }) => (
  <div className={LabelCSS}>{label}</div>
);

const InputPort: React.FC<{
  input: presentation.InputPort;
}> = ({ input }) => (
  <p>
    <Handle
      type="target"
      position={Position.Left}
      id={input.name}
      className={PortCSS}
    />
    {input.name}
  </p>
);

const InputPorts: React.FC<{
  inputs: presentation.InputPort[];
}> = ({ inputs }) => (
  <div className={InputPortsCSS}>
    {inputs.map((input) => (
      <InputPort key={input.name} input={input} />
    ))}
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

const OutputPort: React.FC<{
  output: presentation.OutputPort;
}> = ({ output }) => (
  <p>
    {output.name}
    <Handle
      className={PortCSS}
      type="source"
      position={Position.Right}
      id={output.name}
    />
  </p>
);

const OutputPorts: React.FC<{
  outputs: presentation.OutputPort[];
}> = ({ outputs }) => (
  <div className={OutputPortsCSS}>
    {outputs.map((output) => (
      <OutputPort key={output.name} output={output} />
    ))}
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

export const PropertiesNode: React.FC<
  NodeProps<{
    label: string;
    inputs: presentation.InputPort[];
    outputs: presentation.OutputPort[];
  }>
> = ({ data: { label, inputs, outputs } }) => (
  <div className={PropertiesNodeCSS}>
    <Label label={label} />
    <InputPorts inputs={inputs} />
    <OutputPorts outputs={outputs} />
  </div>
);
