import { useCallback, useMemo, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import type { Node as RFNode, Edge as RFEdge, Connection, HandleType } from 'reactflow';

import {
  useNodesState,
  useEdgesState,
  addEdge,
  updateEdge
} from 'reactflow';

import { PropertiesNode } from "./PropertiesNode";

export type Pipeline = {
  nodes: Node[],
  edges: Edge[],
};

export type Node = RFNode<NodeData>;

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

export type Edge = RFEdge<EdgeData>;

export type EdgeData = {};

export function usePipeline() {
  const edgeUpdateSuccessful = useRef(true);

  const nodeTypes = useMemo(() => ({
    TwitchSubscriber: PropertiesNode,
    TwitchPublisher: PropertiesNode,
  }), []);
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
    if (!edgeUpdateSuccessful.current) {
      setEdges((edges) => edges.filter((e) => e.id !== target.id));
    }
    edgeUpdateSuccessful.current = true;
  }, []);

  const onConnect = useCallback(
    (connection: Connection) => setEdges((edges) => addEdge(connection, edges)),
    [setEdges],
  );

  const onApply = useCallback(() => {
    (async () => {
      await invoke<Pipeline>("update_pipeline", { updated: { nodes, edges } });
    })()
  }, [nodes, edges]);

  useEffect(() => {
    (async () => {
      const { nodes, edges } = await invoke<Pipeline>("pipeline");
      setNodes(nodes);
      setEdges(edges);
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
    onApply,
  };
}
