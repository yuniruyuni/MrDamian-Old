import { Handle, Position } from 'reactflow';

export const PropertiesNode: React.FC<{}> = ({}) => (
  <div style={{
    background: '#000',
    border: '1px solid #000',
    minWidth: '300px',
    display: 'grid',
    gridTemplateRows: '30px auto',
    gridTemplateColumns: '1fr 1fr',
    gap: '1px',
    borderRadius: '5px',
  }}>
    <div style={{
      background: '#fff',
      gridRow: '1 / 2',
      gridColumn: '1 / 3',
      textAlign: 'left',
      verticalAlign: 'baseline',
      borderRadius: '5px 5px 0 0',
    }}>
      Twitch
    </div>
    <div style={{
      background: '#fff',
      gridRow: '2 / 3',
      gridColumn: '1 / 2',
      textAlign: 'left',
      verticalAlign: 'baseline',
      borderRadius: '0 0 0 5px',
    }}>
      <p>
        <Handle type="target" position={Position.Left} id="in-a" style={{
          display: 'inline-block',
          position: 'relative',
        }} />
        trigger
      </p>
    </div>
    <div style={{
      background: '#fff',
      gridRow: '2 / 3',
      gridColumn: '2 / 3',
      textAlign: 'right',
      verticalAlign: 'baseline',
      borderRadius: '0 0 5px 0',
    }}>
      <p>
        channel
        <Handle
          style={{
            display: 'inline-block',
            position: 'relative',
          }}
          type="source"
          position={Position.Right}
          id="out-a" />
      </p>
      <p>
          latest game
          <Handle style={{
            display: 'inline-block',
            position: 'relative',
          }}
          type="source"
          position={Position.Right}
          id="out-b" />
      </p>
    </div>
  </div>
);
