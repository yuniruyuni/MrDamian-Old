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
