import { useCallback, useMemo, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import type { Node } from 'reactflow';

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

type InputPort = {
  name: string,
};

type OutputPort = {
  name: string,
};

type NodeData = {
  label: string,
  inputs: InputPort[],
  outputs: OutputPort[],
};

function App() {
  const nodeTypes = useMemo(() => ({ propertiesNode: PropertiesNode, }), []);
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const onConnect = useCallback(
    (params: any) => setEdges((edge) => addEdge(params, edge)),
    [setEdges],
  );

  useEffect(() => {
    (async () => {
      const response = await invoke<{id: string, data: NodeData}[]>("nodes");
      const nodes: Node[] = response.map(({id, data}) => ({
        type: "propertiesNode",
        id,
        data,
        position: { x: 0, y: 0 },
      }));
      setNodes(nodes);
    })()
  }, []);

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
