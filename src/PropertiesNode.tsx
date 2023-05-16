import type { NodeProps } from 'reactflow';

import { Handle, Position } from 'reactflow';
import { css } from '@acab/ecsstatic';

import { InputPort as Input, OutputPort as Output } from './bindings';

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


const Label: React.FC<{label: string}> = ({ label }) => (
  <div className={LabelCSS}>
    {label}
  </div>
);

const InputPort: React.FC<{input: Input, onAssignEdit?: (i: Input) => void}> = ({input, onAssignEdit}) => (
  <p onClick={() => onAssignEdit && onAssignEdit(input)}>
    <Handle type="target" position={Position.Left} id={input.name} className={PortCSS} />
    {input.name}
  </p>
);

const InputPorts: React.FC<{inputs: Input[], onAssignEdit?: (i: Input) => void}> = ({inputs, onAssignEdit}) => (
  <div className={InputPortsCSS}>
    {inputs.map(input => (<InputPort input={input} onAssignEdit={onAssignEdit} />))}
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

const OutputPort: React.FC<{output: Output, onAssignEdit?: (o: Output) => void}> = ({output, onAssignEdit}) => (
  <p onClick={() => {
    onAssignEdit && onAssignEdit(output);
    console.log(onAssignEdit);
  }}>
    {output.name}
    <Handle className={PortCSS} type="source" position={Position.Right} id={output.name} />
  </p>
);

const OutputPorts: React.FC<{outputs: Output[], onAssignEdit: (o: Output) => void}> = ({outputs, onAssignEdit}) => (
  <div className={OutputPortsCSS}>
    {outputs.map(output => (<OutputPort output={output} onAssignEdit={onAssignEdit} />))}
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
  onInputAssignEdit: (i: Input) => void,
  onOutputAssignEdit: (o: Output) => void,
}>> = ({ data: { label, inputs, outputs, onInputAssignEdit, onOutputAssignEdit } }) => (
  <div className={PropertiesNodeCSS}>
    <Label label={label} />
    <InputPorts inputs={inputs} onAssignEdit={onInputAssignEdit} />
    <OutputPorts outputs={outputs} onAssignEdit={onOutputAssignEdit} />
  </div>
);
