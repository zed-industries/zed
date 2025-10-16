import { mutation, query } from "./_generated/server";
import { v } from "convex/values";

export const get = query({
  handler: async ({ db }) => {
    return await db.query("activity").collect();
  },
});

export const create_session = mutation({
  args: {
    name: v.string(),
    file_name: v.string(),
    class_name: v.string(),
    function_name: v.string(),
    repo_name: v.string(),
  },
  handler: async (ctx, args) => {
    console.log("messsage from the server");
    await ctx.db.update("activity", {
      name: args.name,
      file_name: args.file_name,
      class_name: args.class_name,
      function_name: args.function_name,
      repo_name: args.repo_name,
    });
  },
});

export const update = mutation({
  args: {
    name: v.string(),
    file_name: v.string(),
    class_name: v.string(),
    function_name: v.string(),
    repo_name: v.string(),
  },
  handler: async (ctx, args) => {
    console.log("message from the server");
    const doc = await ctx.db
      .query("activity")
      .filter((q) => q.eq(q.field("name"), args.name))
      .first();

    if (!doc) {
      return;
    }

    await ctx.db.patch(doc._id, {
      name: args.name,
      file_name: args.file_name,
      class_name: args.class_name,
      function_name: args.function_name,
      repo_name: args.repo_name,
    });
  },
});

export const sendReadmePreview = mutation({
  args: {
    name: v.string(),
    readme_preview: v.string(),
  },
  handler: async (ctx, args) => {
    const doc = await ctx.db
      .query("activity")
      .filter((q) => q.eq(q.field("name"), args.name))
      .first();

    if (!doc) {
      return;
    }

    await ctx.db.patch(doc._id, {
      readme_preview: args.readme_preview,
    });
  },
});
