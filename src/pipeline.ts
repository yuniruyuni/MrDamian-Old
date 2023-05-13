import { useCallback, useMemo, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { updatePipeline, pipeline, Node, Edge } from "./bindings";

import type { Node as RFNode, Edge as RFEdge, Connection, HandleType } from 'reactflow';

import {
  useNodesState,
  useEdgesState,
  addEdge,
  updateEdge
} from 'reactflow';

import { PropertiesNode } from "./PropertiesNode";

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
  const onEdgeUpdate = useCallback((oldEdge: RFEdge, newConnection: Connection) => {
    edgeUpdateSuccessful.current = true;
    setEdges((edges) => updateEdge(oldEdge, newConnection, edges));
  }, []);
  const onEdgeUpdateEnd = useCallback((_event: MouseEvent | TouchEvent, target: RFEdge, _handle: HandleType) => {
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
      const rnodes: Node[] = nodes.map((node) => ({
        ...node,
        type: node.type??'',
      }));
      const redges: Edge[] = edges.map((edge) => ({
        ...edge,
        label: edge.label?.toString()??'',
        sourceHandle: edge.sourceHandle??'',
        targetHandle: edge.targetHandle??'',
      }));

      await updatePipeline({ nodes: rnodes, edges: redges });
    })()
  }, [nodes, edges]);

  useEffect(() => {
    (async () => {
      const { nodes, edges } = await pipeline();
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
