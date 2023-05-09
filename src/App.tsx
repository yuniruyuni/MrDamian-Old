import { useCallback, useMemo, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import type { Node, Edge, Connection, HandleType } from 'reactflow';

import ReactFlow, {
  MiniMap,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  updateEdge
} from 'reactflow';

import 'reactflow/dist/style.css';

import { PropertiesNode } from "./PropertiesNode";

import type { NodeData } from "./pipeline";

function App() {
  const edgeUpdateSuccessful = useRef(true);

  const nodeTypes = useMemo(() => ({ propertiesNode: PropertiesNode, }), []);
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  const onEdgeUpdateStart = useCallback(() => {
    edgeUpdateSuccessful.current = false;
  }, []);
  const onEdgeUpdate = useCallback((oldEdge: Edge, newConnection: Connection) => {
    edgeUpdateSuccessful.current = true;
    setEdges((edges) => updateEdge(oldEdge, newConnection, edges));
  }, []);
  const onEdgeUpdateEnd = useCallback((_event: MouseEvent | TouchEvent, target: Edge, _handle: HandleType) => {
    if(!edgeUpdateSuccessful.current) {
      setEdges((edges) => edges.filter((e) => e.id !== target.id));
    }
    edgeUpdateSuccessful.current = true;
  }, []);

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
        onEdgeUpdateStart={onEdgeUpdateStart}
        onEdgeUpdate={onEdgeUpdate}
        onEdgeUpdateEnd={onEdgeUpdateEnd}
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
