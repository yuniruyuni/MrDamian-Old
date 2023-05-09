import { useCallback, useMemo, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import type { Node, Edge, Connection, HandleType } from 'reactflow';

import {
  useNodesState,
  useEdgesState,
  addEdge,
  updateEdge
} from 'reactflow';

import { PropertiesNode } from "./PropertiesNode";

export type InputPort = {
  name: string,
};

export type OutputPort = {
  name: string,
};

export type NodeData = {
  label: string,
  inputs: InputPort[],
  outputs: OutputPort[],
};

export function usePipeline() {
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
    (connection: Connection) => setEdges((edge) => addEdge(connection, edge)),
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

  return {
    nodes,
    edges,
    nodeTypes,
    onNodesChange,
    onEdgesChange,
    onEdgeUpdateStart,
    onEdgeUpdate,
    onEdgeUpdateEnd,
    onConnect,
  };
}
