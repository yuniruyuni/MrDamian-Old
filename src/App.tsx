import { useCallback, useMemo } from "react";
// import { invoke } from "@tauri-apps/api/tauri";

import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  Handle,
  Position,
  useNodesState,
  useEdgesState,
  addEdge
} from 'reactflow';

import 'reactflow/dist/style.css';

const PropertiesNode: React.FC<{}> = ({}) => (
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

const initialEdges = [
  {id: 'e1-2', source: '1', target: '2'},
];

const initialNodes = [
  { id: '1', data: { label: '1' }, position: { x: 0, y: 0 } },
  { id: '2', data: { label: '2' }, position: { x: 0, y: 100 } },
  { id: '3', data: { label: '3' }, position: { x: 0, y: 200 }, type: 'propertiesNode' },
];

function App() {
  const nodeTypes = useMemo(() => ({ propertiesNode: PropertiesNode, }), []);
  const [nodes, setNodes, onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const onConnect = useCallback(
    (params: any) => setEdges((edge) => addEdge(params, edge)),
    [setEdges],
  );

  return (
    <div className="container">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
      >
        <MiniMap />
        <Controls />
        <Background />
      </ReactFlow>
    </div>
  );
}

export default App;
