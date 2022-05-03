/**
 * Welcome to Cloudflare Workers! This is your first worker.
 *
 * - Run `wrangler dev src/index.ts` in your terminal to start a development server
 * - Open a browser tab at http://localhost:8787/ to see your worker in action
 * - Run `wrangler publish src/index.ts --name my-worker` to publish your worker
 *
 * Learn more at https://developers.cloudflare.com/workers/
 */

/// <reference types="@cloudflare/workers-types" />
import type { Env } from "@cloudflare/workers-types"


export default {
  async fetch(request: Request, env: Env, ctx: ExecutionContext): Promise<Response> {
    try {
      const value = await env.BUCKET.get('file.json');
      if (value === null) {
        return new Response('Object not found', { status: 404 })
      }
      return new Response(value.body)
    } catch (e) {
      console.log(e)
      // Workaround for bug in R2 bindings
      return new Response('Object not found', { status: 404 })
    }
  },
};
