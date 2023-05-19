import { mdxAnnotations } from 'mdx-annotations'
import remarkGfm from 'remark-gfm'

export const remarkPlugins = [mdxAnnotations.remark, remarkGfm]
