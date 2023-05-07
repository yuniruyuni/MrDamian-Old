import { useCallback, useMemo } from "react";
// import { invoke } from "@tauri-apps/api/tauri";

import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge
} from 'reactflow';

import 'reactflow/dist/style.css';

import { PropertiesNode } from "./PropertiesNode";

const initialEdges = [
  {id: 'e1-2', source: '1', target: '2'},
];

const initialNodes = [
  {
    id: '1',
    data: {
      label: 'araara',
      inputs: [{name: 'ara-a'}],
      outputs: [{name: 'ara-b'}],
    },
    position: { x: 50, y: 0 },
    type: 'propertiesNode'
  },
  {
    id: '2',
    data: {
      label: 'nipanipa',
      inputs: [{name: 'nipa-a'}],
      outputs: [{name: 'nipa-b'}, {name: 'nipa-c'}],
    },
    position: { x: 50, y: 100 },
    type: 'propertiesNode'
  },
  {
    id: '3',
    data: {
      label: 'yuniruyuni',
      inputs: [{name: 'prop-a'}, {name: 'prop-b'}],
      outputs: [{name: 'prop-c'}],
    },
    position: { x: 50, y: 200 },
    type: 'propertiesNode'
  },
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
