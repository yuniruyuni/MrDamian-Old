import { Handle, Position } from 'reactflow';
import { css } from '@acab/ecsstatic';

const TitleCSS = css`
  background: #fff;
  grid-row: 1 / 2;
  grid-column: 1 / 3;
  text-align: left;
  vertical-align: baseline;
  border-radius: 5px 5px 0 0;
`;

const Title: React.FC<{}> = ({}) => (
  <div className={TitleCSS}>
    Twitch
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

const InputPorts: React.FC<{}> = ({}) => (
  <div className={InputPortsCSS}>
    <p>
      <Handle type="target" position={Position.Left} id="in-a" className={PortCSS} />
      trigger
    </p>
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

const OutputPorts: React.FC<{}> = ({}) => (
  <div className={OutputPortsCSS}>
    <p>
      channel
      <Handle className={PortCSS} type="source" position={Position.Right} id="out-a" />
    </p>
    <p>
      latest game
      <Handle className={PortCSS} type="source" position={Position.Right} id="out-b" />
    </p>
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

export const PropertiesNode: React.FC<{}> = ({}) => (
  <div className={PropertiesNodeCSS}>
    <Title />
    <InputPorts />
    <OutputPorts />
  </div>
);
